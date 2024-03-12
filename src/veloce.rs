use crate::kernel::*;

pub struct Veloce {
    mounts: Package,
    listen: Vec<StdTcpListener>,
}

impl Default for Veloce {
    fn default() -> Self {
        Self { mounts: Package::new(vec![Catcher::default()]), listen: vec![], }
    }
}

impl Veloce {
    pub fn mount<T: Handler>(&mut self, handler: T) -> &mut T {
        self.mounts.insert(handler)
    }

    pub fn force<T: Default + Handler>(&mut self) -> &mut T {
        self.mounts.ensure::<T>()
    }

    pub fn limit(&mut self) -> &mut Limiter {
        self.force()
    }

    pub fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) -> &mut Routine {
        self.force::<Matcher>().add(pattern, handler)
    }

    pub fn group(&mut self, pattern: impl Into<Pattern>) -> &mut Veloce {
        self.route(pattern, Veloce::default()).trust::<Veloce>()
    }

    pub fn catch(&mut self, handler: impl Fn(anyhow::Error) -> Response<Body> + Send + Sync + 'static) {
        match self.mounts.iter_mut::<Catcher>().next() {
            Some(obj) => obj.handler = Box::new(handler),
            _ => unreachable!()
        }
    }

    pub fn bind(&mut self, listener: impl IntoListener) -> Result<&mut Self> {
        self.listen.push(listener.into_listener()?);
        Ok(self)
    }

    pub async fn run(mut self) -> Result<()> {
        use hyper::Server;
        use hyper::server::conn::AddrStream;
        use hyper::service::{make_service_fn, service_fn};

        for handler in self.mounts.iter_mut_all() {
            handler.warmup().await?;
        }

        self.warmup().await?;

        let mut sockets = std::mem::take(&mut self.listen);
        let appself = Arc::new(self);
        let service = make_service_fn(|conn: &AddrStream| {
            let appself = appself.clone();
            let address = (conn.local_addr(), conn.remote_addr());

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let appself = appself.clone();
                    let context = Context::new(appself.clone(), req, address.0, address.1);

                    async move { appself.handle(context).await }
                }))
            }
        });

        let mut servers = vec![];

        while let Some(socket) = sockets.pop() {
            servers.push(Server::from_tcp(socket)?.serve(service));
        }

        futures::future::join_all(servers).await;

        Ok(())
    }
}

#[async_trait]
impl Handler for Veloce {
    async fn warmup(&mut self) -> Result<()> {
        self.mounts.warmup().await
    }

    async fn handle(&self, ctx: Context) -> Result<Response<Body>> {
        self.mounts.handle(ctx).await
    }
}
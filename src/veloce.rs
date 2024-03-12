use crate::kernel::*;

pub struct Veloce {
    mounted: Package,
    sockets: Vec<socket2::Socket>,
}

impl Default for Veloce {
    fn default() -> Self {
        Self { mounted: Package::new(vec![Catcher::default()]), sockets: vec![] }
    }
}

impl Veloce {
    pub fn mount<T: Handler>(&mut self, handler: T) -> &mut T {
        self.mounted.insert(handler)
    }

    pub fn force<T: Handler + Default>(&mut self) -> &mut T {
        self.mounted.ensure()
    }

    pub fn limit(&mut self) -> &mut Limiter {
        self.force()
    }

    pub fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) -> &mut Routine {
        self.force::<Matcher>().add(pattern, handler)
    }

    pub fn group(&mut self, pattern: impl Into<Pattern>) -> &mut Veloce {
        self.route(pattern, Veloce::default()).trust()
    }

    pub fn catch(&mut self, handler: impl Fn(&Catcher, anyhow::Error) -> Response<Body> + Send + Sync + 'static) {
        match self.mounted.first::<Catcher>() {
            Some(obj) => obj.handler = Box::new(handler),
            _ => unreachable!()
        }
    }

    pub fn bind(&mut self, listener: impl IntoListener) -> Result<&mut socket2::Socket> {
        self.sockets.push(listener.into_listener()?);

        match self.sockets.last_mut() {
            Some(obj) => Ok(obj),
            _ => unreachable!()
        }
    }

    pub async fn run(mut self) -> Result<()> {
        use hyper::Server;
        use hyper::server::conn::AddrStream;
        use hyper::service::{make_service_fn, service_fn};

        let mut sockets = std::mem::take(&mut self.sockets);
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
            servers.push(Server::from_tcp(socket.into())?.serve(service));
        }

        futures::future::join_all(servers).await;

        Ok(())
    }
}

#[async_trait]
impl Handler for Veloce {
    async fn handle(&self, ctx: Context) -> Result<Response<Body>> {
        self.mounted.handle(ctx).await
    }
}
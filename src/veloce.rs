use crate::kernel::*;

pub struct Veloce {
    cached: Vec<Box<dyn Handler>>,
    mounts: Arc<Vec<Box<dyn Handler>>>,
    listen: Vec<StdTcpListener>,
}

impl Default for Veloce {
    fn default() -> Self {
        Self {
            cached: vec![Box::<Catcher>::default()],
            mounts: Arc::new(vec![]),
            listen: vec![],
        }
    }
}

impl Veloce {
    pub fn mount<T: Handler>(&mut self, handler: T) -> &mut T {
        if self.cached.last_mut().and_then(|last| last.as_any_mut().downcast_mut::<T>()).is_none() {
            self.cached.push(Box::new(handler));
        }

        match self.cached.last_mut().and_then(|last| last.as_any_mut().downcast_mut::<T>()) {
            Some(handler) => handler,
            None => unreachable!()
        }
    }

    pub fn visit(&mut self) -> std::slice::IterMut<'_, Box<dyn Handler>> {
        self.cached.iter_mut()
    }

    pub fn catch(&mut self, handler: impl Fn(&mut Context, anyhow::Error) + Send + Sync + 'static) {
        match self.cached.last_mut().and_then(|last| last.as_any_mut().downcast_mut::<Catcher>()) {
            Some(catcher) => catcher.handler = Box::new(handler),
            None => unreachable!()
        }
    }

    pub fn bind(&mut self, addr: impl ToSocketAddrs) -> Result<&mut Self> {
        Ok(self.take(StdTcpListener::bind(addr)?))
    }

    pub fn take(&mut self, tcp: StdTcpListener) -> &mut Self {
        self.listen.push(tcp);
        self
    }

    pub async fn run(mut self) -> Result<()> {
        use hyper::Server;
        use hyper::server::conn::AddrStream;
        use hyper::service::{make_service_fn, service_fn};

        for handler in &mut self.cached {
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
                    let mut context = Context::new(appself.clone(), req, address.0, address.1);

                    async move {
                        match appself.handle(&mut context).await {
                            Ok(_) => Ok::<_, Infallible>(context.res),
                            Err(_) => unreachable!()
                        }
                    }
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
        self.mounts = Arc::new(std::mem::take(&mut self.cached));
        Ok(())
    }

    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        ctx.push(self.mounts.clone(), 0);
        ctx.next().await
    }
}
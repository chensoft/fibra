use crate::kernel::*;

pub struct Veloce {
    pub mounts: Vec<Box<dyn Handler>>,
    pub listen: Vec<StdTcpListener>,
}

impl Default for Veloce {
    fn default() -> Self {
        Self {
            mounts: vec![Box::<Catcher>::default()],
            listen: vec![],
        }
    }
}

impl Veloce {
    pub fn mount<T: Handler>(&mut self, handler: T) -> &mut T {
        if self.mounts.last_mut().and_then(|last| last.as_any_mut().downcast_mut::<T>()).is_none() {
            self.mounts.push(Box::new(handler));
        }

        match self.mounts.last_mut().and_then(|last| last.as_any_mut().downcast_mut::<T>()) {
            Some(handler) => handler,
            None => unreachable!()
        }
    }

    pub fn catch(&mut self, handler: impl Fn(&mut Context, anyhow::Error) + Send + Sync + 'static) {
        match self.mounts.first_mut().and_then(|first| first.as_any_mut().downcast_mut::<Catcher>()) {
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

        let mut sockets = std::mem::take(&mut self.listen);
        let appself = Arc::new(self);
        let service = make_service_fn(|conn: &AddrStream| {
            let appself = appself.clone();
            let address = (conn.local_addr(), conn.remote_addr());

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let appself = appself.clone();
                    let context = Context::new(appself.clone(), req, address.0, address.1);

                    async move {
                        // todo
                        // match appself.call(&mut context).await {
                        //     Ok(_) => Ok::<_, Infallible>(context.res),
                        //     Err(_) => unreachable!()
                        // }
                        Ok::<_, Infallible>(Response::new(Body::default()))
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
    async fn next(&self, mut ctx: Context) -> Result<()> {
        match ctx.step() {
            Some(idx) => self.mounts[idx].call(ctx).await,
            None => Ok(())
        }
    }

    async fn call(&self, mut ctx: Context) -> Result<()> {
        ctx.push(self.mounts.len());
        self.next(ctx).await
    }
}
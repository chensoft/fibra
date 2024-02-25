use crate::addons;
use crate::consts::*;
use crate::kernel::*;

pub struct Veloce {
    router: Matcher,
    addons: Vec<Arc<dyn Handler>>,
    listen: Vec<StdTcpListener>,
}

impl Veloce {
    pub fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        self.router.add(pattern, handler);
    }

    pub fn group(&mut self, pattern: impl Into<Pattern>, initial: impl Fn(&mut Veloce)) {
        let mut veloce = Veloce::default();
        initial(&mut veloce);
        self.route(pattern, veloce);
    }

    pub fn mount(&mut self, handler: impl Handler) {
        self.addons.push(Arc::new(handler));
    }

    pub async fn bind(&mut self, addr: &str) -> Result<()> {
        match tokio::net::lookup_host(addr).await {
            Ok(mut ret) => match ret.next() {
                None => Err(Error::HostNotFound("DNS resolution list is empty".into()).into()),
                Some(addr) => self.take(StdTcpListener::bind(addr)?),
            },
            Err(err) => Err(Error::HostNotFound(err.to_string()).into()),
        }
    }

    pub fn take(&mut self, tcp: StdTcpListener) -> Result<()> {
        self.listen.push(tcp);
        Ok(())
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
                    let mut context = Context::new(appself.clone(), req.into(), Address::new(address.0, address.1));

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

impl Default for Veloce {
    fn default() -> Self {
        Self {router: Default::default(), addons: vec![Arc::new(addons::Catcher::default())], listen: vec![]}
    }
}

#[async_trait]
impl Handler for Veloce {
    async fn handle(&self, ctx: &mut Context) -> Result<()> {
        match self.router.get(ctx.search.as_str()) {
            Some(val) => ctx.routes.push_front(val),
            None => return Err(StatusCode::NOT_FOUND.into_error()),
        };

        // todo subrouter
        ctx.parent = ctx.search.clone();
        ctx.search = "/".to_string();

        for handler in self.addons.iter().rev() {
            ctx.routes.push_front(handler.clone());
        }

        ctx.next().await
    }
}
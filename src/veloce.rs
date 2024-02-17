use crate::plugin;
use crate::consts::*;
use crate::kernel::*;

#[derive(Default)]
pub struct Veloce {
    pub config: Config,
    routes: Vec<Arc<dyn Handler>>,
    listen: Vec<StdTcpListener>,
}

impl Veloce {
    pub fn new(config: Config) -> Self {
        Self {config, routes: vec![], listen: vec![]}
    }

    pub fn mount(&mut self, handler: impl Handler) {
        self.routes.push(Arc::new(handler));
    }

    pub fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        let last = self.routes.last_mut().map(|val| (val as &mut dyn Any).downcast_mut::<Matcher>()).flatten();

        match last {
            Some(matcher) => {
                matcher.add(pattern, handler);
            }
            None => {
                let mut matcher = Matcher::new();
                matcher.add(pattern, handler);
                self.routes.push(Arc::new(matcher));
            }
        }
    }

    pub fn group(&mut self, pattern: impl Into<Pattern>, config: Config) -> &mut Veloce {
        self.route(pattern, Veloce::new(config));
        match self.routes.last_mut().map(|val| (val as &mut dyn Any).downcast_mut::<Veloce>()).flatten() {
            Some(val) => val,
            None => unreachable!()
        }
    }

    pub fn public(&mut self, pattern: impl Into<Pattern>, folder: PathBuf, config: Option<Static>) {
        self.route(pattern, plugin::Public::new(folder, config));
    }

    pub fn reject(&mut self, pattern: impl Into<Pattern>, status: Option<http::StatusCode>) {
        self.route(pattern, plugin::Reject::new(status));
    }

    pub fn rewrite(&mut self, from: impl Into<Pattern>, to: impl Into<http::Uri>) {
        self.route(from, plugin::Rewrite::new(to));
    }

    pub fn redirect(&mut self, from: impl Into<Pattern>, to: impl Into<http::Uri>, status: Option<http::StatusCode>) {
        self.route(from, plugin::Redirect::new(to, status));
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
        use http::{Response, Server};
        use http::server::conn::AddrStream;
        use http::service::{make_service_fn, service_fn};

        let mut sockets = std::mem::take(&mut self.listen);
        let appself = Arc::new(self);
        let service = make_service_fn(|conn: &AddrStream| {
            let appself = appself.clone();
            let address = (conn.local_addr(), conn.remote_addr());

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let context = Context {
                        app: appself.clone(),
                        req,
                        res: Response::default(),
                        sock: address.0,
                        peer: address.1,
                        cache: Storage,
                        chain: VecDeque::new(),
                    };
                    let appself = appself.clone();

                    async move {
                        let res = match AssertUnwindSafe(appself.handle(context)).catch_unwind().await {
                            Ok(ret) => match ret {
                                Ok(ctx) => ctx.res,
                                Err(err) => (appself.config.catch)(err),
                            }
                            Err(err) => match err.downcast_ref::<&str>() {
                                Some(err) => (appself.config.catch)(Error::Panicked(err.to_string()).into()),
                                None => (appself.config.catch)(Error::Panicked("Unknown error".to_string()).into()),
                            }
                        };

                        Ok::<_, Infallible>(res)
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
    async fn handle(&self, mut ctx: Context) -> Result<Context> {
        for handler in &self.routes {
            ctx.chain.push_front(handler.clone());
        }

        ctx.next().await
    }
}
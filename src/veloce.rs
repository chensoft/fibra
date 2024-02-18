use crate::plugin;
use crate::consts::*;
use crate::kernel::*;

#[derive(Default)]
pub struct Veloce {
    pub config: Config,
    router: Matcher,
    plugin: Vec<Arc<dyn Handler>>,
    listen: Vec<StdTcpListener>,
}

impl Veloce {
    pub fn new(config: Config) -> Self {
        Self {config, ..Default::default()}
    }

    pub fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        self.router.add(pattern, handler);
    }

    pub fn group(&mut self, pattern: impl Into<Pattern>, config: Option<Config>, setup: fn(&mut Veloce)) {
        let mut veloce = Veloce::new(config.unwrap_or_default());
        setup(&mut veloce);
        self.route(pattern, veloce);
    }

    pub fn public(&mut self, pattern: impl Into<Pattern>, folder: PathBuf, config: Option<Static>) {
        self.route(pattern, plugin::Public::new(folder, config));
    }

    pub fn reject(&mut self, pattern: impl Into<Pattern>, status: Option<http::StatusCode>) {
        self.route(pattern, plugin::Reject::new(status));
    }

    pub fn rewrite(&mut self, from: impl Into<Pattern>, to: http::Uri) {
        self.route(from, plugin::Rewrite::new(to));
    }

    pub fn redirect(&mut self, from: impl Into<Pattern>, to: http::Uri, status: Option<http::StatusCode>) {
        self.route(from, plugin::Redirect::new(to, status));
    }

    pub fn mount(&mut self, handler: impl Handler) {
        self.plugin.push(Arc::new(handler));
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
                    let rawpath = req.uri().path().to_string();
                    let context = Context {
                        app: appself.clone(),
                        req,
                        res: Response::default(),
                        sock: address.0,
                        peer: address.1,
                        temp: Storage::default(),
                        routes: VecDeque::new(),
                        parent: "".to_string(),
                        search: rawpath,
                    };
                    let appself = appself.clone();

                    async move {
                        // todo method not found
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
        match self.router.get(ctx.search.as_str()) {
            Some(val) => ctx.routes.push_front(val),
            None => return Err(Error::RouteNotFound(ctx.req.uri().to_string()).into()),
        };

        // todo subrouter
        ctx.parent = ctx.search.clone();
        ctx.search = "/".to_string();

        for handler in self.plugin.iter().rev() {
            ctx.routes.push_front(handler.clone());
        }

        ctx.next().await
    }
}
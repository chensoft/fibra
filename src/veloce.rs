use crate::config::*;
use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Veloce {
    config: Config,
    routes: Vec<Box<dyn Handler>>, // todo METHOD/PATTERN/OTHER radix tree key use pattern indextree, if not found method goto methodnotallowed config, if pattern goto notfound config
    listen: Vec<StdTcpListener>,
}

impl Veloce {
    pub fn new(config: Option<Config>) -> Self {
        Self {
            config: config.unwrap_or_default(),
            routes: vec![],
            listen: vec![],
        }
    }

    pub fn mount(&mut self, handler: impl Handler) {
        self.routes.push(Box::new(handler));
    }

    // todo .route("/xxx", handler).method(GET, POST, CUSTOM)
    pub fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        // todo pattern filter
        
    }

    pub fn group(&mut self, pattern: impl Into<Pattern>, config: Option<Config>) -> &mut Self {
        todo!()
    }

    pub fn public(&mut self, pattern: impl Into<Pattern>, directory: PathBuf, presets: Option<Public>) {

    }

    pub fn reject(&mut self, pattern: impl Into<Pattern>) {

    }

    pub fn rewrite(&mut self, from: impl Into<Pattern>, to: impl Into<Pattern>) {

    }

    pub fn redirect(&mut self, from: impl Into<Pattern>, to: impl Into<Pattern>, _code: http::StatusCode) {

    }

    pub async fn bind(&mut self, addr: &str) -> Result<()> {
        match tokio::net::lookup_host(addr).await {
            Ok(mut ret) => match ret.next() {
                None => Err(Error::DNSFailed("DNS resolution list is empty".into()).into()),
                Some(addr) => self.take(StdTcpListener::bind(addr)?),
            },
            Err(err) => Err(Error::DNSFailed(err.to_string()).into()),
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
            let _appself = appself.clone();
            let address = (conn.local_addr(), conn.remote_addr());

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let context = Context {
                        req,
                        res: Response::default(),
                        sock: address.0,
                        peer: address.1,
                        temp: Storage,
                    };

                    // for handler in &appself.handlers {
                    //     handler.handle(context);
                    // }

                    async move {
                        Ok::<_, Infallible>(context.res)
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
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
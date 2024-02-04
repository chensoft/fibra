#[macro_use] extern crate thiserror;
#[macro_use] extern crate async_trait;

pub use http;
pub use mime;

pub mod filter;
pub mod plugin;

mod context;
mod general;
mod handler;
mod pattern;
mod storage;

pub use context::*;
pub use general::*;
pub use handler::*;
pub use pattern::*;
pub use storage::*;

pub struct Veloce {
    presets: Preset,
    filters: Vec<Box<dyn filter::Filter>>, // todo METHOD/PATTERN/OTHER radix tree key use pattern indextree, if not found method goto methodnotallowed config, if pattern goto notfound config
    plugins: Vec<Box<dyn plugin::Plugin>>,
    sockets: Vec<StdTcpListener>,
}

impl Veloce {
    pub fn new(presets: Option<Preset>) -> Self {
        Self {
            presets: presets.unwrap_or_default(),
            filters: vec![],
            plugins: vec![],
            sockets: vec![],
        }
    }

    pub fn mount(&mut self, plugin: impl plugin::Plugin) -> &mut Self {
        self.plugins.push(Box::new(plugin));
        self
    }

    // todo .route("/xxx", handler).method(GET, POST, CUSTOM)
    pub fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) -> &mut Self {
        // todo pattern filter
        self
    }

    pub fn group(&mut self, pattern: impl Into<Pattern>, presets: Option<Preset>) -> Self {
        Self::new(presets)
    }

    pub fn public(&mut self, pattern: impl Into<Pattern>, directory: PathBuf, presets: Option<Public>) -> &mut Self {
        self
    }

    pub fn reject(&mut self, pattern: impl Into<Pattern>) -> &mut Self {
        self
    }

    pub fn rewrite(&mut self, from: impl Into<Pattern>, to: impl Into<Pattern>) -> &mut Self {
        self
    }

    pub fn redirect(&mut self, from: impl Into<Pattern>, to: impl Into<Pattern>, _code: http::StatusCode) -> &mut Self {
        self
    }

    pub async fn bind(&mut self, addr: &str) -> Result<&mut Self> {
        match tokio::net::lookup_host(addr).await {
            Ok(mut ret) => match ret.next() {
                None => Err(Error::DNSFailed(Cow::Borrowed("DNS resolution list is empty")).into()),
                Some(addr) => self.take(StdTcpListener::bind(addr)?),
            },
            Err(err) => Err(Error::DNSFailed(Cow::Owned(err.to_string())).into()),
        }
    }

    pub fn take(&mut self, tcp: StdTcpListener) -> Result<&mut Self> {
        self.sockets.push(tcp);
        Ok(self)
    }

    pub async fn run(mut self) -> Result<()> {
        use http::{Response, Server};
        use http::server::conn::AddrStream;
        use http::service::{make_service_fn, service_fn};

        let mut sockets = std::mem::take(&mut self.sockets);
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
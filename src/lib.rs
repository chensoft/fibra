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
    _config: Config,
    handlers: Vec<Box<dyn Handler>>,
    services: Vec<StdTcpListener>,
}

impl Veloce {
    pub fn new(config: Option<Config>) -> Self {
        Self {
            _config: config.unwrap_or_default(),
            handlers: vec![],
            services: vec![],
        }
    }

    pub fn sub(&mut self, _pattern: &str) -> Self {
        todo!()
    }

    pub fn route(&mut self, _pattern: &str, handler: impl Handler) -> &mut Self {
        // todo pattern filter
        self.mount(handler);
        self
    }

    pub fn mount(&mut self, handler: impl Handler) -> &mut Self {
        self.handlers.push(Box::new(handler));
        self
    }

    pub fn public(&mut self, _pattern: &str, _dir: PathBuf, _conf: Option<Public>) -> &mut Self {
        self
    }

    pub fn reject(&mut self, _pattern: &str) -> &mut Self {
        self
    }

    pub fn rewrite(&mut self, _from: &str, _to: &str) -> &mut Self {
        self
    }

    pub fn redirect(&mut self, _from: &str, _to: &str, _code: http::StatusCode) -> &mut Self {
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
        self.services.push(tcp);
        Ok(self)
    }

    pub async fn run(mut self) -> Result<()> {
        use http::{Response, Server};
        use http::server::conn::AddrStream;
        use http::service::{make_service_fn, service_fn};

        let mut sockets = std::mem::take(&mut self.services);
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
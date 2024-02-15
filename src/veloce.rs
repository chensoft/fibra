use crate::plugin;
use crate::config::*;
use crate::consts::*;
use crate::traits::*;
use crate::kernel::*;

pub struct Veloce {
    config: Config,
    routes: Vec<Box<dyn AnyHandler>>,
    listen: Vec<StdTcpListener>,
}

impl Default for Veloce {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Veloce {
    pub fn new(config: Option<Config>) -> Self {
        Self {config: config.unwrap_or_default(), routes: vec![], listen: vec![]}
    }

    pub fn mount(&mut self, handler: impl Handler) {
        self.routes.push(Box::new(handler));
    }

    pub fn route(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        let last = self.routes.last_mut().map(|val| val.as_any_mut().downcast_mut::<Matcher>()).flatten();

        match last {
            Some(matcher) => {
                matcher.add(pattern, handler);
            }
            None => {
                let mut matcher = Matcher::default();
                matcher.add(pattern, handler);
                self.routes.push(Box::new(matcher));
            }
        }
    }

    pub fn group(&mut self, pattern: impl Into<Pattern>, config: Option<Config>) -> &mut Veloce {
        self.route(pattern, Veloce::new(config));
        match self.routes.last_mut().map(|val| val.as_any_mut().downcast_mut::<Veloce>()).flatten() {
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

    pub fn catch(&mut self, error: fn(Context, anyhow::Error) -> http::Response<http::Body>) {
        self.config.error = error;
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
            let appself = appself.clone();
            let address = (conn.local_addr(), conn.remote_addr());

            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    let appself = appself.clone();
                    let mut context = Context {
                        req,
                        res: Response::default(),
                        sock: address.0,
                        peer: address.1,
                        temp: Storage,
                    };

                    async move {
                        match AssertUnwindSafe(context.next()).catch_unwind().await {
                            Ok(ret) => match ret {
                                Ok(_) => Ok::<_, Infallible>(context.res),
                                Err(err) => Ok::<_, Infallible>((appself.config.error)(context, err)),
                            }
                            Err(err) => {
                                if let Some(err) = err.downcast_ref::<&str>() {
                                    Ok::<_, Infallible>((appself.config.error)(context, anyhow!(err.clone())))
                                } else if let Some(err) = err.downcast_ref::<String>() {
                                    Ok::<_, Infallible>((appself.config.error)(context, anyhow!(err.clone())))
                                } else {
                                    Ok::<_, Infallible>((appself.config.error)(context, anyhow!("unknown panic")))
                                }
                            }
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
    async fn handle(&self, ctx: Context) -> Result<()> {
        todo!()
    }
}
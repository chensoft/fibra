use super::types::*;
use super::route::*;
use super::context::*;
use super::handler::*;

pub struct Veloce {
    config: Config,
    routes: Vec<Route>,  // todo alias for Box<dyn Future<Output = ()>
}

impl Veloce {
    pub fn new(config: Option<Config>) -> Self {
        Self {
            config: config.unwrap_or_default(),
            routes: vec![],
        }
    }

    pub fn sub(&mut self, _pattern: &str) -> Self { todo!() }

    pub fn route(&mut self, pattern: &str, handler: impl Handler + 'static) -> &mut Self {
        self.routes.push(Route {pattern: pattern.into(), handler: Box::new(handler)});
        self
    }

    // pub fn mount(&mut self, _handler: Handler) {}
    pub fn public(&mut self, _pattern: &str, _dir: PathBuf, _conf: Option<Public>) {}
    pub fn reject(&mut self, _pattern: &str) {}
    pub fn rewrite(&mut self, _from: &str, _to: &str) {}
    pub fn redirect(&mut self, _from: &str, _to: &str, _code: http::StatusCode) {}

    pub fn take(&mut self, _tcp: i32) -> Result<&mut Self> {
        Ok(self)
    }

    pub async fn bind(&mut self, _addr: impl tokio::net::ToSocketAddrs) -> Result<&mut Self> {
        Ok(self)
    }

    pub async fn run(self) -> Result<()> {
        Ok(())
    }
}
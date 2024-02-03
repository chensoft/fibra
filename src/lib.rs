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
    routes: Vec<Box<dyn Handler>>,
}

impl Veloce {
    pub fn new(config: Option<Config>) -> Self {
        Self {
            _config: config.unwrap_or_default(),
            routes: vec![],
        }
    }

    pub fn sub(&mut self, _pattern: &str) -> Self { todo!() }

    pub fn route(&mut self, _pattern: &str, handler: impl Handler) -> &mut Self {
        self.routes.push(Box::new(handler));
        self
    }

    pub fn mount(&mut self, _handler: impl Handler) {

    }

    pub fn public(&mut self, _pattern: &str, _dir: PathBuf, _conf: Option<Public>) {
    }

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
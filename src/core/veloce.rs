use super::define::*;
use super::router::*;

pub struct Veloce {
    
}

impl Veloce {
    pub fn new(_conf: Option<Config>) -> Self {
        Self {}
    }

    pub fn take(&mut self, _tcp: i32) -> anyhow::Result<&mut Self> {
        Ok(self)
    }

    pub async fn bind(&mut self, _addr: impl tokio::net::ToSocketAddrs) -> anyhow::Result<&mut Self> {
        Ok(self)
    }
    pub async fn run(&mut self, _root: Router) -> anyhow::Result<()> {
        Ok(())
    }
}
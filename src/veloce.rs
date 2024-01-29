use super::config::*;
use super::router::*;

pub struct Veloce;

impl Veloce {
    pub fn new(_config: Option<Config>) -> Veloce {
        todo!()
    }

    pub fn bind(&mut self) {}
    pub fn run(&mut self, _root: Router) {}
}
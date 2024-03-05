use crate::veloce::*;
use crate::kernel::*;

pub trait Domains {
    fn domains(&mut self, domains: Vec<impl Into<Pattern>>) -> &mut Self;
}

impl Domains for Veloce {
    fn domains(&mut self, domains: Vec<impl Into<Pattern>>) -> &mut Self {
        todo!()
    }
}
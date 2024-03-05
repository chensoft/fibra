use crate::veloce::*;
use crate::kernel::*;

pub trait Domains {
    fn domain(&mut self, domain: impl Into<Pattern>) -> &mut Self;
    fn domains(&mut self, domains: Vec<impl Into<Pattern>>) -> &mut Self;
}

impl Domains for Veloce {
    fn domain(&mut self, domain: impl Into<Pattern>) -> &mut Self {
        todo!()
    }

    fn domains(&mut self, domains: Vec<impl Into<Pattern>>) -> &mut Self {
        todo!()
    }
}
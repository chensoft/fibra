use crate::consts::*;
use crate::veloce::*;
use crate::kernel::*;

pub trait Methods {
    fn methods(&mut self, methods: Vec<Method>) -> &mut Self;
}

impl Methods for Veloce {
    fn methods(&mut self, methods: Vec<Method>) -> &mut Self {
        todo!()
    }
}
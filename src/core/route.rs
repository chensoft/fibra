use super::handler::*;

pub struct Route {
    pub pattern: String,
    pub handler: Box<dyn Handler>,
}
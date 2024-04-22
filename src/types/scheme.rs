#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Scheme {
    HTTP,
    HTTPS,
}
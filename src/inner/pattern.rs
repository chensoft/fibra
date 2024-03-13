// todo regex wildcard
#[derive(Default, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pattern;

impl Pattern {
    pub fn matches(&self, _val: &str) -> bool {
        false
    }
}

impl From<&str> for Pattern {
    fn from(_value: &str) -> Self {
        // todo
        Self {}
    }
}

impl From<String> for Pattern {
    fn from(_value: String) -> Self {
        // todo
        Self {}
    }
}
// todo regex wildcard
pub struct Pattern;

impl Pattern {
    pub fn matches(&self, val: &str) -> bool {
        false
    }
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        todo!()
    }
}

impl From<String> for Pattern {
    fn from(value: String) -> Self {
        todo!()
    }
}
// todo regex wildcard
#[derive(Default)]
pub struct Pattern;

impl Pattern {
    pub fn matches(&self, _val: &str) -> bool {
        false
    }
}

impl From<&str> for Pattern {
    fn from(_value: &str) -> Self {
        todo!()
    }
}

impl From<String> for Pattern {
    fn from(_value: String) -> Self {
        todo!()
    }
}
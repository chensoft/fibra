use crate::consts::*;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum Pattern {
    Plain(String),
    Param(String),
    Regex(String),
}

impl From<&str> for Pattern {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<String> for Pattern {
    fn from(value: String) -> Self {
        // todo param and regex
        Pattern::Plain(value)
    }
}

impl Equivalent<Pattern> for &str {
    fn equivalent(&self, key: &Pattern) -> bool {
        match key {
            Pattern::Plain(prefix) => prefix == self,
            Pattern::Param(prefix) => prefix == self, // todo
            Pattern::Regex(prefix) => prefix == self, // todo
        }
    }
}
use crate::consts::*;
use crate::kernel::*;

#[derive(Default)]
pub struct Matcher {
    pub preway: IndexMap<Pattern, Arc<dyn Handler>> // todo multiple handler
}

impl Matcher {
    pub fn add(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        let pattern = pattern.into();
        match self.preway.get_mut(&pattern) {
            None => { self.preway.insert(pattern, Arc::new(handler)); }
            Some(val) => *val = Arc::new(handler),
        }
    }

    pub fn get(&self, uri: &str) -> Option<Arc<dyn Handler>> {
        self.preway.get(&Pattern::Plain(uri.to_string())).cloned()
    }
}
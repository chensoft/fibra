use crate::consts::*;
use crate::kernel::*;

#[derive(Default)]
pub struct Matcher {
    pub preway: HashMap<Pattern, Arc<dyn Handler>> // todo multiple handler
}

impl Matcher {
    pub fn add(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        let pattern = pattern.into();
        match self.preway.get_mut(&pattern) {
            None => { self.preway.insert(pattern, Arc::new(handler)); }
            Some(val) => *val = Arc::new(handler),
        }
    }

    pub fn get(&self, path: &str) -> Option<Arc<dyn Handler>> {
        self.preway.get(path).cloned()
    }
}
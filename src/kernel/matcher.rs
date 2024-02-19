use crate::consts::*;
use crate::kernel::*;

#[derive(Default)]
pub struct Matcher {
    pub preway: HashMap<String, HashMap<Pattern, Arc<dyn Handler>>> // todo multiple handler
}

impl Matcher {
    pub fn add(&mut self, method: impl Into<Method>, pattern: impl Into<Pattern>, handler: impl Handler) {
        let pattern = pattern.into();
        let storage = self.preway.entry(method.into().into_owned()).or_insert(HashMap::new());

        match storage.get_mut(&pattern) {
            None => { storage.insert(pattern, Arc::new(handler)); }
            Some(val) => *val = Arc::new(handler),
        }
    }

    pub fn get(&self, method: &str, path: &str) -> Option<Arc<dyn Handler>> {
        match self.preway.get(method) {
            Some(map) => map.get(path).cloned(),
            None => None
        }
    }
}
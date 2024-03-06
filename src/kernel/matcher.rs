use crate::kernel::*;

#[derive(Default)]
pub struct Matcher {
    pub preway: HashMap<Pattern, Arc<dyn Handler>> // todo multiple handler
}

impl Matcher {
    pub fn new(pattern: impl Into<Pattern>, handler: impl Handler) -> Self {
        let mut obj = Self::default();
        obj.add(pattern, handler);
        obj
    }

    pub fn add(&mut self, pattern: impl Into<Pattern>, handler: impl Handler) {
        // let pattern = pattern.into();
        // match self.preway.get_mut(&pattern) {
        //     None => { self.preway.insert(pattern, Arc::new(handler)); }
        //     Some(val) => *val = Arc::new(handler),
        // }
        todo!()
    }
}

#[async_trait]
impl Handler for Matcher {
    async fn handle(&self, _ctx: &mut Context) -> Result<()> {
        todo!()
    }
}
use crate::kernel::*;

#[derive(Default)]
pub struct Matcher {
    pub preway: HashMap<Pattern, Vec<Routine>>
}

impl Matcher {
    pub fn add(&mut self, _pattern: impl Into<Pattern>, _handler: impl Handler) -> &mut Routine {
        todo!()
    }
}

#[async_trait]
impl Handler for Matcher {
    async fn handle(&self, _ctx: Context) -> Result<Response<Body>> {
        todo!()
    }
}
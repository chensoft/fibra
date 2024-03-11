use crate::kernel::*;

#[derive(Default)]
pub struct Package {
    cached: Vec<Box<dyn Handler>>,
    bundle: Arc<Vec<Box<dyn Handler>>>,
}

impl Package {
    pub fn new(handlers: Vec<impl Handler>) -> Self {
        Self { cached: handlers.into_iter().map(|item| Box::new(item) as Box<dyn Handler>).collect(), bundle: Arc::new(vec![]) }
    }

    pub fn add<T: Handler>(&mut self, handler: impl Handler) -> &mut T {
        self.cached.push(Box::new(handler));
        match self.iter_mut::<T>().last() {
            Some(Some(obj)) => obj,
            _ => unreachable!()
        }
    }

    pub fn iter<T: Handler>(&mut self) -> impl Iterator<Item = Option<&T>> {
        self.cached.iter().map(|handler| handler.as_ref().as_any().downcast_ref::<T>())
    }

    pub fn iter_mut<T: Handler>(&mut self) -> impl Iterator<Item = Option<&mut T>> {
        self.cached.iter_mut().map(|handler| handler.as_mut().as_any_mut().downcast_mut::<T>())
    }

    pub fn iter_all(&mut self) -> impl Iterator<Item = &Box<dyn Handler>> {
        self.cached.iter()
    }

    pub fn iter_mut_all(&mut self) -> impl Iterator<Item = &mut Box<dyn Handler>> {
        self.cached.iter_mut()
    }
}

#[async_trait]
impl Handler for Package {
    async fn warmup(&mut self) -> Result<()> {
        self.bundle = Arc::new(std::mem::take(&mut self.cached));
        Ok(())
    }

    async fn handle(&self, mut ctx: Context) -> Result<Response<Body>> {
        ctx.push(self.bundle.clone(), 0);
        ctx.next().await
    }
}
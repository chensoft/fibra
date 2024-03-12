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

    pub fn insert<T: Handler>(&mut self, handler: impl Handler) -> &mut T {
        self.cached.push(Box::new(handler));
        match self.iter_mut::<T>().last() {
            Some(obj) => obj,
            _ => unreachable!()
        }
    }

    pub fn ensure<T: Default + Handler>(&mut self) -> &mut T {
        if self.iter::<T>().last().is_none() {
            return self.insert(T::default());
        }

        match self.iter_mut::<T>().last() {
            Some(obj) => obj,
            _ => unreachable!()
        }
    }

    pub fn bundle(&self) -> &Vec<Box<dyn Handler>> {
        &self.cached // todo
    }

    pub fn iter<T: Handler>(&mut self) -> impl Iterator<Item = &T> {
        self.cached.iter().map(|handler| handler.as_ref().as_any().downcast_ref::<T>()).flatten()
    }

    pub fn iter_mut<T: Handler>(&mut self) -> impl Iterator<Item = &mut T> {
        self.cached.iter_mut().map(|handler| handler.as_mut().as_any_mut().downcast_mut::<T>()).flatten()
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
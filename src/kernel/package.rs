use crate::kernel::*;

#[derive(Default)]
pub struct Package {
    pub bundle: Vec<BoxHandler>,
}

impl Package {
    pub fn new(handlers: Vec<impl Handler>) -> Self {
        Self { bundle: handlers.into_iter().map(|item| Box::new(item) as BoxHandler).collect() }
    }

    pub fn insert<T: Handler>(&mut self, handler: impl Handler) -> &mut T {
        self.bundle.push(Box::new(handler));

        match self.last::<T>() {
            Some(obj) => obj,
            _ => unreachable!()
        }
    }

    pub fn ensure<T: Handler + Default>(&mut self) -> &mut T {
        if self.last::<T>().is_none() {
            return self.insert(T::default());
        }

        match self.last::<T>() {
            Some(obj) => obj,
            _ => unreachable!()
        }
    }

    pub fn first<T: Handler>(&mut self) -> Option<&mut T> {
        self.bundle.first_mut().and_then(|h| h.as_mut().as_any_mut().downcast_mut::<T>())
    }

    pub fn last<T: Handler>(&mut self) -> Option<&mut T> {
        self.bundle.last_mut().and_then(|h| h.as_mut().as_any_mut().downcast_mut::<T>())
    }
}

#[async_trait]
impl Handler for Package {
    fn nested(&self, idx: usize) -> Option<&BoxHandler> {
        self.bundle.get(idx)
    }

    async fn handle(&self, mut ctx: Context) -> Result<Response<Body>> {
        ctx.push(self);
        ctx.next().await
    }
}
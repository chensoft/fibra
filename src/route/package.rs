use crate::route::*;
use crate::types::*;

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
        self.last::<T>().unwrap_or_else(|| unreachable!())
    }

    pub fn ensure<T: Handler + Default>(&mut self) -> &mut T {
        if self.last::<T>().is_none() {
            return self.insert(T::default());
        }

        self.last::<T>().unwrap_or_else(|| unreachable!())
    }

    pub fn first<T: Handler>(&mut self) -> Option<&mut T> {
        self.bundle.first_mut().and_then(|h| h.as_handler_mut::<T>())
    }

    pub fn last<T: Handler>(&mut self) -> Option<&mut T> {
        self.bundle.last_mut().and_then(|h| h.as_handler_mut::<T>())
    }
}

#[async_trait]
impl Handler for Package {
    fn nested(&self, idx: usize) -> Option<&BoxHandler> {
        self.bundle.get(idx)
    }

    async fn handle(&self, mut ctx: Context) -> FibraResult<Response> {
        ctx.push(self);
        ctx.next().await
    }
}
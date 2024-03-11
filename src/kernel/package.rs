use crate::kernel::*;

#[derive(Default)]
pub struct Package {
    cached: Vec<Box<dyn Handler>>,
    bundle: Arc<Vec<Box<dyn Handler>>>,
}

impl Package {
    pub fn add(&mut self, handler: impl Handler) -> &mut Self {
        self.cached.push(Box::new(handler));
        self
    }

    pub fn iter<T: Handler>(&self) -> PackageIter<T> {
        PackageIter::new(self)
    }

    pub fn iter_mut<T: Handler>(&mut self) -> PackageIterMut<T> {
        PackageIterMut::new(self)
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

pub struct PackageIter<'a, T: Handler> {
    object: &'a Package,
    cursor: usize,
    marker: PhantomData<T>,
}

impl<'a, T: Handler> PackageIter<'a, T> {
    fn new(object: &'a Package) -> Self {
        PackageIter { object, cursor: 0, marker: PhantomData }
    }
}

impl<'a, T: Handler> Iterator for PackageIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cursor < self.object.cached.len() {
            if let Some(item) = self.object.cached[self.cursor].as_ref().as_any().downcast_ref::<T>() {
                self.cursor += 1;
                return Some(item);
            }

            self.cursor += 1;
        }

        None
    }
}

pub struct PackageIterMut<'a, T: Handler> {
    object: &'a mut Package,
    cursor: usize,
    marker: PhantomData<T>,
}

impl<'a, T: Handler> PackageIterMut<'a, T> {
    fn new(object: &'a mut Package) -> Self {
        PackageIterMut { object, cursor: 0, marker: PhantomData }
    }
}

impl<'a, T: Handler> Iterator for PackageIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cursor < self.object.cached.len() {
            if let Some(item) = self.object.cached[self.cursor].as_mut().as_any_mut().downcast_mut::<T>() {
                self.cursor += 1;
                let ptr: *mut T = item;
                return unsafe { Some(&mut *ptr) };
            }

            self.cursor += 1;
        }

        None
    }
}
//! Route Matcher
use crate::route::*;
use crate::types::*;

/// A route matcher determines which handler to invoke for an incoming HTTP request based on
/// the request's URL.
#[derive(Default)]
pub struct Matcher {
    routes: RadixMap<Vec<Routine>>
}

impl Matcher {
    /// Create a new object
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inert a new route into the matcher
    pub fn insert(&mut self, path: impl Into<Bytes>, handler: impl Handler) -> FibraResult<&mut Routine> {
        let path = path.into();

        if !self.routes.contains_key(path.as_ref()) {
            self.routes.insert(path.clone(), vec![])?;
        }

        let list = self.routes.raw_mut(path.as_ref()).unwrap_or_else(|| unreachable!());
        list.push(Routine::from(handler));

        Ok(list.last_mut().unwrap_or_else(|| unreachable!()))
    }
}

#[async_trait]
impl Handler for Matcher {
    async fn handle(&self, mut ctx: Context) -> FibraResult<Response> {
        if let (Some(routes), params) = self.routes.capture(ctx.path().as_bytes()) {
            if !params.is_empty() {
                let new: IndexMap<String, String> = params.into_iter().map(|(k, v)| {
                    (unsafe { String::from_utf8_unchecked(k.to_vec()) }, unsafe { std::str::from_utf8_unchecked(v).to_string() })
                }).collect();

                ctx.params_mut().extend(new);
            }

            return routes.handle(ctx).await;
        }

        ctx.next().await
    }
}
/// Serve
///
/// ```no_run
/// use veloce::*;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     serve!("0.0.0.0:3000", "0.0.0.0:3333"; "/" => get!(hello))
/// }
///
/// async fn hello(ctx: &mut Context) -> Result<()> {
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! serve {
    ($($listen:literal),+; $($pattern:literal => $handler:expr),+) => {{
        let mut app = $crate::Veloce::default();
        $(app.route($pattern, $handler);)+
        $(app.bind($listen)?;)+
        app.run().await
    }};
}

#[macro_export]
macro_rules! all {
    ($func:expr) => {{
        $crate::addons::Closure::new(|ctx: &mut Context| Box::pin(async move {
            $func(ctx).await
        }))
    }};
}

// todo multiple
#[macro_export]
macro_rules! get {
    ($func:expr) => {{
        $crate::addons::Closure::new(|ctx: &mut Context| Box::pin(async move {
            match ctx.req.method() == $crate::Method::GET {
                true => $func(ctx).await,
                false => ctx.next().await,
            }
        }))
    }};
}
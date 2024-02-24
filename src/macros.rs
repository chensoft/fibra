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
        $(app.bind($listen).await?;)+
        app.run().await
    }};
}

// todo multiple
#[macro_export]
macro_rules! any {
    ($func:expr) => {{
        $crate::Closure::new(|ctx: &mut Context| Box::pin(async move {
            $func(ctx).await
        }))
    }};
}

// todo multiple
#[macro_export]
macro_rules! get {
    ($func:expr) => {{
        $crate::addons::Router::new($crate::Method::GET, $crate::Closure::new(|ctx: &mut Context| Box::pin(async move {
            $func(ctx).await
        })))
    }};
}
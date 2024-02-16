/// Serve
/// 
/// ```no_run
/// use veloce::{Context, Result};
/// 
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     veloce::serve!("0.0.0.0:3000", "0.0.0.0:3333"; "/" => hello)
/// }
/// 
/// async fn hello(mut ctx: Context) -> Result<()> {
///     Ok(ctx)
/// }
/// ```
#[macro_export]
macro_rules! serve {
    ($($listen:literal),+; $($pattern:literal => $handler:expr),+) => {{
        let mut app = $crate::Veloce::new(None);
        $(app.route($pattern, $handler);)+
        $(app.bind($listen).await?;)+
        app.run().await
    }};
}
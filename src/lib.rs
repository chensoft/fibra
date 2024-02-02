#[macro_use] extern crate async_trait;

pub mod core;
pub mod exts;

pub use http;
pub use mime;
pub use core::*;

#[macro_export]
macro_rules! serve {
    ($($listen:literal),+; $($pattern:literal => $handler:expr),+) => {{
        let mut app = $crate::Veloce::new(None);
        $(app.route($pattern, $crate::exts::func::wrap($handler));)+
        $(app.bind($listen).await?;)+
        app.run().await
    }};
}
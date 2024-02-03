pub(crate) use std::borrow::Cow;
pub(crate) use std::path::PathBuf;
pub(crate) use std::future::Future;
pub(crate) use std::net::SocketAddr;

pub type Result<T> = anyhow::Result<T>;

#[macro_export]
macro_rules! serve {
    ($($listen:literal),+; $($pattern:literal => $handler:expr),+) => {{
        let mut app = $crate::Veloce::new(None);
        $(app.route($pattern, $crate::filter::Func::new($handler));)+
        $(app.bind($listen).await?;)+
        app.run().await
    }};
}

#[derive(Debug, Default, Clone)]
pub struct Config {

}

#[derive(Debug, Default, Clone)]
pub struct Public {

}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {

}
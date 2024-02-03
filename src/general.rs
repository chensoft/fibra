pub(crate) use std::sync::Arc;
pub(crate) use std::borrow::Cow;
pub(crate) use std::path::PathBuf;
pub(crate) use std::future::Future;
pub(crate) use std::net::SocketAddr;
pub(crate) use std::convert::Infallible;
pub(crate) use std::net::TcpListener as StdTcpListener;

pub type Result<T> = anyhow::Result<T>;

#[macro_export]
macro_rules! serve {
    ($($listen:literal),+; $($pattern:literal => $handler:expr),+) => {{
        let mut app = $crate::Veloce::new(None);
        $(app.route($pattern, $crate::route!($handler));)+
        $(app.bind($listen).await?;)+
        app.run().await
    }};
}

#[macro_export]
macro_rules! route {
    ($handler:expr) => {{
        $crate::filter::Func::new($handler)
    }};
}

#[derive(Debug, Default, Clone)]
pub struct Config {

}

#[derive(Debug, Default, Clone)]
pub struct Public {

}

#[derive(Debug, Clone, Error, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {
    #[error("{0}")]
    DNSFailed(Cow<'static, str>),
}
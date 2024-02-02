pub(crate) use std::borrow::Cow;
pub(crate) use std::path::PathBuf;
pub(crate) use std::future::Future;
pub(crate) use std::net::SocketAddr;

// pub(crate) use indexmap::IndexMap;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug, Default, Clone)]
pub struct Config {

}

#[derive(Debug, Default, Clone)]
pub struct Public {

}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Error {

}
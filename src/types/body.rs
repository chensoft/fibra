//! HTTP Body
use crate::types::*;
use http_body_util::Full;
use http_body_util::BodyExt;

pub(crate) type BoxBody = http_body_util::combinators::BoxBody<Bytes, FibraError>;

/// HTTP Body
#[derive(Default)]
pub struct Body(BoxBody);

impl Body {
    /// Create a new object
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Read all bytes
    #[inline]
    pub async fn read_all(&mut self) -> BufList {
        let mut list = BufList::new();
        while let Some(bytes) = self.read_frame().await {
            list.push_chunk(bytes);
        }
        list
    }

    /// Read one frame
    #[inline]
    pub async fn read_frame(&mut self) -> Option<Bytes> {
        self.0.frame().await?.ok().and_then(|frame| frame.into_data().ok())
    }
}

impl From<Body> for BoxBody {
    #[inline]
    fn from(value: Body) -> Self {
        value.0
    }
}

impl From<()> for Body {
    #[inline]
    fn from(_: ()) -> Self {
        Self(BoxBody::default())
    }
}

impl From<BoxBody> for Body {
    #[inline]
    fn from(value: BoxBody) -> Self {
        Self(value)
    }
}

impl From<Bytes> for Body {
    #[inline]
    fn from(value: Bytes) -> Self {
        Self(Full::new(value).map_err(|err| match err {}).boxed())
    }
}

impl From<&'static str> for Body {
    #[inline]
    fn from(value: &'static str) -> Self {
        Self(Full::from(value).map_err(|err| match err {}).boxed())
    }
}

impl From<String> for Body {
    #[inline]
    fn from(value: String) -> Self {
        Self(Full::from(value).map_err(|err| match err {}).boxed())
    }
}

impl From<&'static [u8]> for Body {
    #[inline]
    fn from(value: &'static [u8]) -> Self {
        Self(Full::from(value).map_err(|err| match err {}).boxed())
    }
}

impl From<Vec<u8>> for Body {
    #[inline]
    fn from(value: Vec<u8>) -> Self {
        Self(Full::from(value).map_err(|err| match err {}).boxed())
    }
}
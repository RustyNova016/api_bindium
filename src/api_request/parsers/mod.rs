use crate::ApiRequest;
use crate::ApiRequestError;

pub mod bytes;
#[cfg(feature = "image")]
pub mod image;
pub mod json;
pub mod text;

/// Parse a response object to a specific type.
///
/// # Examples
///
/// See [text::TextParser], [json::JsonParser]
pub trait Parser<R> {
    type Output;

    fn parse<P>(request: &ApiRequest<P>, response: R) -> Result<Self::Output, ApiRequestError>;
}

#[cfg(feature = "image")]
pub mod image;
pub mod bytes;
pub mod json;
pub mod text;

/// Parse a response object to a specific type.
///
/// # Examples
///
/// See [text::TextParser], [json::JsonParser]
pub trait Parser<R> {
    /// The output type of the parser
    type Output;
    /// The error type of the parser
    type Error;

    fn parse(&self, response: R) -> Result<Self::Output, Self::Error>;
}

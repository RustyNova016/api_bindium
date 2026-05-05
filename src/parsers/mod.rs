pub mod bytes;
#[cfg(feature = "image")]
pub mod image;
pub mod json;
pub mod text;

/// Parse a response object to a specific type.
///
/// Parsers should only handle translating the response to an unique rust type, and not the deserialization details. For exemple:
/// - An api endpoint returning two different bodies depending on the input **should** be handled in the parser
/// - Tweaking the response body to have a nicer rust representation **should not** be handled in the parser. It should be set in the the deserializer
pub trait Parser<R> {
    /// The output type of the parser
    type Output;
    /// The error type of the parser
    type Error;

    fn parse(&self, response: R) -> Result<Self::Output, Self::Error>;
}

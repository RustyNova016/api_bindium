#![allow(
    clippy::result_large_err,
    reason = "Until `snafu` got something to easily box error sources, we ignore it"
)]
// only enables the `doc_cfg` feature when
// the `docsrs` configuration attribute is defined
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod api_request;
pub mod api_response;
pub mod client;
pub mod endpoints;
pub mod error;
pub mod http_verb;
pub mod parsers;
#[cfg(test)]
pub mod tests;
pub mod utils;

pub use crate::client::ApiClient;
pub use crate::error::ApiRequestError;
pub use crate::http_verb::HTTPVerb;
pub use api_request::ApiRequest;

pub use crate::parsers::Parser;
pub use crate::parsers::bytes::ByteParser;
#[cfg(feature = "image")]
pub use crate::parsers::image::ImageParser;
pub use crate::parsers::json::JsonParser;
pub use crate::parsers::text::TextParser;

// === Re-exports ===
#[cfg(feature = "rate_limit")]
pub use governor;
pub use ureq;

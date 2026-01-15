// Until `snafu` got something to easily box error sources, we ignore it
#![allow(clippy::result_large_err)]
// Add feature warnings
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod api_request;
pub mod client;
pub mod endpoints;
pub mod http_verb;
#[cfg(test)]
pub mod tests;
pub mod utils;

pub use crate::api_request::error::ApiRequestError;
pub use crate::client::ApiClient;
pub use crate::http_verb::HTTPVerb;
pub use api_request::ApiRequest;

// === Re-exports ===
#[cfg(feature = "rate_limit")]
pub use governor;
pub use ureq;

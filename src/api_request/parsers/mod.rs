use ureq::Body;
use ureq::http::Response;

use crate::ApiRequestError;

pub mod bytes;
#[cfg(feature = "image")]
pub mod image;
pub mod json;
pub mod text;

pub trait Parser<T> {
    fn parse(response: &mut Response<Body>, max_size: u64) -> Result<T, ApiRequestError>;
}

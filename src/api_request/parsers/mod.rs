use ureq::Body;
use ureq::http::Response;

use crate::ApiRequestError;

pub mod bytes;
pub mod json;
pub mod text;

pub trait Parser<T> {
    fn parse(response: &mut Response<Body>) -> Result<T, ApiRequestError>;
}

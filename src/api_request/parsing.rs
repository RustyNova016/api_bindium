use ureq::Body;
use ureq::http::Response;

use crate::ApiRequest;
use crate::api_request::error::ApiRequestError;

use crate::api_request::parsers::Parser;

impl<P> ApiRequest<P> {
    /// Parse the request json5
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn parse_response<T>(&self, response: &mut Response<Body>) -> Result<T, ApiRequestError>
    where
        P: Parser<T>,
    {
        P::parse(response)
    }
}

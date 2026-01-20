use ureq::Body;
use ureq::http::Response;

use crate::ApiRequest;
use crate::api_request::error::ApiRequestError;

use crate::api_request::parsers::Parser;

impl<P> ApiRequest<P> {
    /// Parse the request json
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn parse_response(&self, response: Response<Body>) -> Result<P::Output, ApiRequestError>
    where
        P: Parser<Response<Body>>,
    {
        P::parse(self, response)
    }
}

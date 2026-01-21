use snafu::ResultExt as _;
use ureq::ResponseExt as _;

use crate::api_request::error::UreqSnafu;
use crate::api_request::parsers::Parser;

pub struct ByteParser;

impl Parser<ureq::http::Response<ureq::Body>> for ByteParser {
    type Output = Vec<u8>;

    fn parse<P>(
        request: &crate::ApiRequest<P>,
        mut response: ureq::http::Response<ureq::Body>,
    ) -> Result<Self::Output, crate::ApiRequestError> {
        response
            .body_mut()
            .with_config()
            .limit(request.max_body_size())
            .read_to_vec()
            .context(UreqSnafu {
                uri: response.get_uri().to_owned(),
            })
    }
}

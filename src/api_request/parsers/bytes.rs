use snafu::ResultExt as _;
use ureq::ResponseExt as _;

use crate::api_request::error::UreqSnafu;
use crate::api_request::parsers::Parser;

pub struct ByteParser;

impl Parser<Vec<u8>> for ByteParser {
    fn parse(
        response: &mut ureq::http::Response<ureq::Body>,
        max_size: u64,
    ) -> Result<Vec<u8>, crate::ApiRequestError> {
        response
            .body_mut()
            .with_config()
            .limit(max_size)
            .read_to_vec()
            .context(UreqSnafu {
                uri: response.get_uri().to_owned(),
            })
    }
}

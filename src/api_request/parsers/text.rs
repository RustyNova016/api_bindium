use snafu::ResultExt as _;
use ureq::ResponseExt as _;

use crate::api_request::error::UreqSnafu;
use crate::api_request::parsers::Parser;

pub struct TextParser;

impl Parser<String> for TextParser {
    fn parse(
        response: &mut ureq::http::Response<ureq::Body>,
        max_size: u64,
    ) -> Result<String, crate::ApiRequestError> {
        response
            .body_mut()
            .with_config()
            .limit(max_size)
            .read_to_string()
            .context(UreqSnafu {
                uri: response.get_uri().to_owned(),
            })
    }
}

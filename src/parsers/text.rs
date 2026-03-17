use snafu::ResultExt as _;
use ureq::ResponseExt as _;

use crate::api_response::ureq_response::UreqResponseInner;
use crate::error::UreqSnafu;
use crate::parsers::Parser;

/// Parse the response into a plain [`String`]
pub struct TextParser;

impl Parser<UreqResponseInner> for TextParser {
    type Output = String;
    type Error = crate::ApiRequestError;

    fn parse(&self, mut response: UreqResponseInner) -> Result<Self::Output, Self::Error> {
        response
            .data
            .body_mut()
            .with_config()
            .limit(response.max_body_size)
            .read_to_string()
            .context(UreqSnafu {
                uri: response.data.get_uri().to_owned(),
            })
    }
}

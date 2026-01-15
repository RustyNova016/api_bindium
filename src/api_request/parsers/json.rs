use core::marker::PhantomData;

use serde_core::de::DeserializeOwned;
use snafu::ResultExt as _;

use crate::api_request::error::JsonParsingSnafu;
use crate::api_request::parsers::Parser;
use crate::api_request::parsers::text::TextParser;

pub struct JsonParser<T>(PhantomData<T>)
where
    T: Sized + DeserializeOwned;

impl<T> Parser<T> for JsonParser<T>
where
    T: Sized + DeserializeOwned,
{
    fn parse(
        response: &mut ureq::http::Response<ureq::Body>,
        max_size: u64,
    ) -> Result<T, crate::ApiRequestError> {
        let text = TextParser::parse(response, max_size)?;

        // Try to deserialize as our result
        let err = match serde_json::from_str::<T>(&text) {
            Ok(result) => return Ok(result),
            Err(err) => err,
        };

        // Not a server error? Then it's a problem with our models. Let's send the serde error
        Err(err).with_context(|_| JsonParsingSnafu { data: text })
    }
}

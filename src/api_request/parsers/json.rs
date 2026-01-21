use core::marker::PhantomData;

use serde_core::de::DeserializeOwned;
use snafu::ResultExt as _;

use crate::api_request::error::JsonParsingSnafu;
use crate::api_request::parsers::Parser;
use crate::api_request::parsers::text::TextParser;

pub struct JsonParser<T>(PhantomData<T>)
where
    T: Sized + DeserializeOwned;

impl<T> Parser<ureq::http::Response<ureq::Body>> for JsonParser<T>
where
    T: Sized + DeserializeOwned,
{
    type Output = T;

    fn parse<P>(
        request: &crate::ApiRequest<P>,
        response: ureq::http::Response<ureq::Body>,
    ) -> Result<Self::Output, crate::ApiRequestError> {
        let text = TextParser::parse(request, response)?;

        // Try to deserialize as our result
        let err = match serde_json::from_str::<T>(&text) {
            Ok(result) => return Ok(result),
            Err(err) => err,
        };

        // Not a server error? Then it's a problem with our models. Let's send the serde error
        Err(err).with_context(|_| JsonParsingSnafu { data: text })
    }
}

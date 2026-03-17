use core::marker::PhantomData;

use serde_core::de::DeserializeOwned;
use snafu::ResultExt as _;

use crate::TextParser;
use crate::api_response::ureq_response::UreqResponseInner;
use crate::error::JsonParsingSnafu;
use crate::parsers::Parser;

/// Parse a json response into T
#[derive(Debug)]
pub struct JsonParser<T>(PhantomData<T>)
where
    T: Sized + DeserializeOwned;

impl<T> Parser<UreqResponseInner> for JsonParser<T>
where
    T: Sized + DeserializeOwned,
{
    type Output = T;
    type Error = crate::ApiRequestError;

    fn parse(&self, response: UreqResponseInner) -> Result<Self::Output, Self::Error> {
        let text = TextParser.parse(response)?;

        // Try to deserialize as our result
        let err = match serde_json::from_str::<T>(&text) {
            Ok(result) => return Ok(result),
            Err(err) => err,
        };

        // Not a server error? Then it's a problem with our models. Let's send the serde error
        Err(err).with_context(|_| JsonParsingSnafu { data: text })
    }
}

impl<T> Default for JsonParser<T>
where
    T: Sized + DeserializeOwned,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

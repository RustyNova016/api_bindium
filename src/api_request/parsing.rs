use serde::de::DeserializeOwned;
use snafu::ResultExt;
use ureq::Body;
use ureq::ResponseExt;
use ureq::http::Response;

use crate::ApiRequest;
use crate::api_request::error::ApiRequestError;
use crate::api_request::error::InvalidResponseSnafu;
use crate::api_request::error::UreqSnafu;

impl<T> ApiRequest<T> {
    /// Parse the request json5
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn parse_response(&self, response: &mut Response<Body>) -> Result<T, ApiRequestError>
    where
        T: DeserializeOwned,
    {
        let text = response.body_mut().read_to_string().context(UreqSnafu {
            uri: response.get_uri().to_owned(),
        })?;

        // Try to deserialize as our result
        let err = match serde_json::from_str::<T>(&text) {
            Ok(result) => return Ok(result),
            Err(err) => err,
        };

        // Not a server error? Then it's a problem with our models. Let's send the serde error
        Err(err).with_context(|_| InvalidResponseSnafu { data: text })
    }
}

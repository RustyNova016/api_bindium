use core::fmt::Display;
use core::str::FromStr;

use snafu::ResultExt;
use snafu::Snafu;
use ureq::http::Uri;
use ureq::http::uri::InvalidUri;

use crate::ApiRequest;
use crate::HTTPVerb;

pub mod authority;
pub mod path;
pub mod query;
pub mod scheme;

/// Builder to create an URI for an endpoint.
pub struct EndpointUriBuilder<State> {
    uri: String,

    _state: State,
}

impl<S> EndpointUriBuilder<S> {
    pub fn to_uri(self) -> Result<Uri, UriBuilderError> {
        self.try_into()
    }

    pub fn into_api_request<T>(self, verb: HTTPVerb) -> Result<ApiRequest<T>, UriBuilderError> {
        Ok(ApiRequest::builder().uri(self.to_uri()?).verb(verb).build())
    }

    pub fn into_api_request_with_body<T>(
        self,
        verb: HTTPVerb,
        body: serde_json::Value,
    ) -> Result<ApiRequest<T>, UriBuilderError> {
        Ok(ApiRequest::builder()
            .uri(self.to_uri()?)
            .verb(verb)
            .body(body)
            .build())
    }
}

impl<S> Display for EndpointUriBuilder<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}

impl<S> TryFrom<EndpointUriBuilder<S>> for Uri {
    type Error = UriBuilderError;

    fn try_from(value: EndpointUriBuilder<S>) -> Result<Self, Self::Error> {
        Uri::from_str(&value.uri).context(UriBuilderSnafu { uri: value.uri })
    }
}

#[derive(Debug, Snafu)]
#[snafu(display("The built URI has an invalid schema: {uri}"))]
pub struct UriBuilderError {
    source: InvalidUri,

    #[snafu(implicit)]
    location: snafu::Location,

    uri: String,

    #[cfg(feature = "backtrace")]
    backtrace: snafu::Backtrace,
}

use core::fmt::Display;

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
    pub fn to_uri(self) -> Result<Uri, InvalidUri> {
        self.try_into()
    }

    pub fn into_api_request<T>(self, verb: HTTPVerb) -> Result<ApiRequest<T>, InvalidUri> {
        Ok(ApiRequest::builder()
            .uri(self.to_uri()?)
            .verb(verb)
            .build())
    }
}

impl<S> Display for EndpointUriBuilder<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uri)
    }
}

impl<S> TryFrom<EndpointUriBuilder<S>> for Uri {
    type Error = InvalidUri;

    fn try_from(value: EndpointUriBuilder<S>) -> Result<Self, Self::Error> {
        Uri::try_from(value.uri)
    }
}

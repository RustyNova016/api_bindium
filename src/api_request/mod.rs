use core::marker::PhantomData;
use core::time::Duration;
use std::collections::HashMap;
use std::time::Instant;

#[cfg(any(feature = "sync", feature = "async"))]
use ureq::Body;
#[cfg(any(feature = "sync", feature = "async"))]
use ureq::RequestBuilder;
#[cfg(any(feature = "sync", feature = "async"))]
use ureq::http::Response;
use ureq::http::Uri;

use crate::HTTPVerb;

#[cfg(feature = "async")]
pub mod async_funcs;
pub mod error;
pub mod parsers;
pub mod parsing;
#[cfg(feature = "sync")]
pub mod sync_funcs;

/// A raw API request, used to send custom requests to the API
#[derive(Debug, Clone, bon::Builder)]
pub struct ApiRequest<P> {
    /// The uri to fetch
    #[builder(into)]
    uri: Uri,

    /// The http verb of the api request
    verb: HTTPVerb,

    #[builder(default)]
    headers: HashMap<String, String>,

    /// The body of the request
    body: Option<serde_json::Value>,

    /// The parser to use on the response
    #[builder(skip)]
    pub parser: PhantomData<P>,

    // === Fetching state ===
    /// The current number of times the request has been tried
    #[builder(skip)]
    pub tries: u32,

    /// Do not retry the query before this instant
    #[builder(skip = Instant::now())]
    pub retry_after: Instant,
}

impl<T> ApiRequest<T> {
    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn verb(&self) -> HTTPVerb {
        self.verb
    }

    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn body(&self) -> &Option<serde_json::Value> {
        &self.body
    }

    /// Reset the api request back to 0 tries
    pub fn reset(&mut self) {
        self.tries = 0
    }

    /// Increment the retry counter and set the new retry after
    pub fn increment_retry(&mut self, retry_after: Option<Instant>) {
        self.tries += 1;
        if let Some(retry_after) = retry_after {
            self.retry_after = retry_after;
            return;
        }

        let secs_to_wait = self.tries * (self.tries as f32 / 0.5).round() as u32;
        self.retry_after = Instant::now() + Duration::from_secs(secs_to_wait as u64)
    }

    /// Add the config to an ureq request
    #[cfg(any(feature = "sync", feature = "async"))]
    pub(super) fn config_req<Bod>(&self, req: RequestBuilder<Bod>) -> RequestBuilder<Bod> {
        let mut req = req.config().http_status_as_error(false).build();

        for (name, value) in &self.headers {
            req = req.header(name, value)
        }

        req
    }
}

#[cfg(any(feature = "sync", feature = "async"))]
fn get_temporary_error_timeout(response: &Response<Body>) -> Option<Instant> {
    let headers = response.headers();

    let retry_after = headers.get("retry-after")?;

    let Ok(retry_after) = retry_after.to_str() else {
        return None;
    };

    let Ok(retry_after) = retry_after.parse::<u64>() else {
        return None;
    };

    Some(Instant::now() + Duration::from_secs(retry_after + 1))
}

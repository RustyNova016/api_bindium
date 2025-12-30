use snafu::ResultExt;
use ureq::Body;
use ureq::http::Response;

use crate::ApiClient;
use crate::ApiRequest;
use crate::ApiRequestError;
use crate::api_request::error::MaxRetriesExceededSnafu;
use crate::api_request::error::UreqSnafu;
use crate::api_request::get_temporary_error_timeout;
use crate::api_request::parsers::Parser;
use crate::utils::sleep_until;

impl<T> ApiRequest<T> {
    /// Send the request without any fluff.
    ///
    /// This is an advanced function. You are probably looking for [Self::send]
    pub fn send_request_raw(&self, client: &ApiClient) -> Result<Response<Body>, ApiRequestError> {
        let uri = self.uri.to_owned();

        #[cfg(feature = "tracing")]
        tracing::debug!(
            "Sending {} request at {uri} (Try {})",
            self.verb,
            self.tries
        );

        match self.verb {
            crate::HTTPVerb::Get => Self::send_without_body(client.agent.get(&uri)),
            crate::HTTPVerb::Post => {
                Self::send_with_body(client.agent.post(&uri), self.body.clone())
            }
        }
        .context(UreqSnafu { uri })
    }

    /// Send the request, deal with errors and ratelimiting
    ///
    /// Returns `Ok(None)` on a retriable error
    ///
    /// This is an advanced function. You are probably looking for [Self::send]
    pub fn try_send_request(
        &mut self,
        client: &ApiClient,
    ) -> Result<Option<Response<Body>>, ApiRequestError> {
        // Wait to be ready
        sleep_until(self.retry_after);

        // `governor` is async. So we cannot ratelimit sync
        // #[cfg(feature = "rate_limit")]
        // client.await_rate_limit().await;

        // Try fetching the api
        let response = match self.send_request_raw(client) {
            Ok(val) => val,
            Err(err) => {
                if err.is_retryable() {
                    self.increment_retry(None);
                    return Ok(None);
                } else {
                    return Err(err);
                }
            }
        };

        // Let's check if we hit the rate limit
        if response.status().as_u16() == 503 {
            let retry_after = get_temporary_error_timeout(&response);
            self.increment_retry(retry_after);

            return Ok(None);
        };

        Ok(Some(response))
    }

    /// Send the request, and retry on failure
    ///
    /// This is an advanced function. You are probably looking for [Self::send]
    pub fn send_with_retries(
        &mut self,
        client: &ApiClient,
    ) -> Result<Response<Body>, ApiRequestError> {
        while self.tries < client.max_retries {
            if let Some(res) = self.try_send_request(client)? {
                return Ok(res);
            }
        }

        MaxRetriesExceededSnafu.fail()
    }

    /// Send the api request with retries and return the parsed data.
    pub fn send<O>(&mut self, client: &ApiClient) -> Result<O, ApiRequestError>
    where
        T: Parser<O>,
    {
        let mut response = self.send_with_retries(client)?;
        self.parse_response(&mut response)
    }
}

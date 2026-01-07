use ureq::Body;
use ureq::http::Response;

use crate::ApiClient;
use crate::ApiRequest;
use crate::ApiRequestError;
use crate::api_request::error::MaxRetriesExceededSnafu;

use crate::api_request::get_temporary_error_timeout;
use crate::api_request::parsers::Parser;
use crate::utils::sleep_until;

impl<T> ApiRequest<T>
where
    T: Sync,
{
    /// Send the request, deal with errors and ratelimiting
    ///
    /// Returns `Ok(None)` on a retriable error
    ///
    /// This is an advanced function. You are probably looking for [Self::send_async]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn try_send_request(
        &mut self,
        client: &ApiClient,
    ) -> Result<Option<Response<Body>>, ApiRequestError> {
        // Wait to be ready
        sleep_until(self.retry_after);

        // Try fetching the api
        let response = match self.convert_and_send(&client.agent) {
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
    /// This is an advanced function. You are probably looking for [Self::send_async]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn send_with_retries(&mut self, client: &ApiClient) -> Result<Response<Body>, ApiRequestError> {
        while self.tries < client.max_retries {
            if let Some(res) = self.try_send_request(client)? {
                return Ok(res);
            }
        }

        MaxRetriesExceededSnafu.fail()
    }

    /// Send the api request with retries and return the parsed data.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub fn send<O>(&mut self, client: &ApiClient) -> Result<O, ApiRequestError>
    where
        T: Parser<O>,
    {
        let mut response = self.send_with_retries(client)?;
        self.parse_response(&mut response)
    }
}

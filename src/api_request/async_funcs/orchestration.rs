use ureq::Body;
use ureq::http::Response;

use crate::ApiClient;
use crate::ApiRequest;
use crate::ApiRequestError;
use crate::api_response::ureq_response::UreqResponse;
use crate::error::MaxRetriesExceededSnafu;

use crate::api_request::get_temporary_error_timeout;
use crate::utils::sleep_until_async;

impl<P> ApiRequest<P>
where
    P: Sync,
{
    /// Send the request, deal with errors and ratelimiting
    ///
    /// Returns `Ok(None)` on a retriable error
    ///
    /// This is an advanced function. You are probably looking for [Self::send_async]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    async fn try_send_request_async(
        &mut self,
        client: &ApiClient,
    ) -> Result<Option<Response<Body>>, ApiRequestError> {
        // Wait to be ready
        sleep_until_async(self.retry_after).await;

        // Wait for the ratelimit
        #[cfg(feature = "rate_limit")]
        client.await_rate_limit().await;

        // Try fetching the api

        let response = match self.convert_and_send_async(&client.agent).await {
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
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub async fn send_async(
        &mut self,
        client: &ApiClient,
    ) -> Result<UreqResponse<P>, ApiRequestError> {
        while self.tries < client.max_retries {
            if let Some(res) = self.try_send_request_async(client).await? {
                return Ok(UreqResponse::new(res, self.max_body_size, self.parser.clone()));
            }
        }

        MaxRetriesExceededSnafu.fail()
    }
}

use serde::de::DeserializeOwned;
use snafu::ResultExt;
use ureq::Body;
use ureq::http::Response;

use crate::ApiClient;
use crate::ApiRequest;
use crate::ApiRequestError;
use crate::api_request::error::MaxRetriesExceededSnafu;
use crate::api_request::error::UreqSnafu;

use crate::api_request::get_temporary_error_timeout;
use crate::utils::sleep_until_async;

impl<T> ApiRequest<T>
where
    T: Sync,
{
    /// Send the request without any fluff.
    ///
    /// This is an advanced function. You are probably looking for [Self::send_async]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub async fn send_request_raw_async(
        &self,
        client: &ApiClient,
    ) -> Result<Response<Body>, ApiRequestError> {
        let uri = self.uri.to_owned();
        let body = self.body.to_owned();
        let agent = client.agent.to_owned();
        let verb = self.verb.clone();

        #[cfg(feature = "tracing")]
        tracing::debug!(
            "Sending {} request at {uri} (Try {})",
            self.verb,
            self.tries
        );

        blocking::unblock(move || {
            match verb {
                crate::HTTPVerb::Get => Self::send_without_body(agent.get(&uri)),
                crate::HTTPVerb::Post => Self::send_with_body(agent.post(&uri), body),
            }
            .context(UreqSnafu { uri })
        })
        .await
    }

    /// Send the request, deal with errors and ratelimiting
    ///
    /// Returns `Ok(None)` on a retriable error
    ///
    /// This is an advanced function. You are probably looking for [Self::send_async]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub async fn try_send_request_async(
        &mut self,
        client: &ApiClient,
    ) -> Result<Option<Response<Body>>, ApiRequestError> {
        // Wait to be ready
        sleep_until_async(self.retry_after).await;

        // Wait for the ratelimit
        #[cfg(feature = "rate_limit")]
        client.await_rate_limit().await;

        // Try fetching the api
        let response = match self.send_request_raw_async(client).await {
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
    pub async fn send_with_retries_async(
        &mut self,
        client: &ApiClient,
    ) -> Result<Response<Body>, ApiRequestError> {
        while self.tries < client.max_retries {
            if let Some(res) = self.try_send_request_async(client).await? {
                return Ok(res);
            }
        }

        MaxRetriesExceededSnafu.fail()
    }

    /// Send the api request with retries and return the parsed data.
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub async fn send_async(&mut self, client: &ApiClient) -> Result<T, ApiRequestError>
    where
        T: DeserializeOwned,
    {
        let mut response = self.send_with_retries_async(client).await?;
        self.parse_response(&mut response)
    }
}

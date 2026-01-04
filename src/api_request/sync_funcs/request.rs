use snafu::ResultExt;
use ureq::Agent;
use ureq::Body;
use ureq::RequestBuilder;
use ureq::http::Response;
use ureq::typestate::WithBody;
use ureq::typestate::WithoutBody;

use crate::ApiRequest;
use crate::ApiRequestError;
use crate::HTTPVerb;
use crate::api_request::error::UreqSnafu;

impl<T> ApiRequest<T> {
    /// Convert the API request into a ureq request and sends it
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub(super) fn convert_and_send(
        &self,
        agent: &Agent,
    ) -> Result<Response<Body>, ApiRequestError> {
        #[cfg(feature = "tracing")]
        tracing::debug!(
            "Sending {} request at {} (Try {})",
            self.verb,
            self.uri,
            self.tries
        );

        match &self.verb {
            HTTPVerb::Get => {
                let req = self.config_req(agent.get(self.uri()));
                Self::send_without_body(req).context(UreqSnafu {
                    uri: self.uri().clone(),
                })
            }

            HTTPVerb::Post => {
                let req = self.config_req(agent.post(self.uri()));
                Self::send_with_body(req, self.body.clone()).context(UreqSnafu {
                    uri: self.uri().clone(),
                })
            }
        }
    }

    /// Send the ureq request in another thread
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn send_without_body(req: RequestBuilder<WithoutBody>) -> Result<Response<Body>, ureq::Error> {
        req.call()
    }

    /// Send the ureq request with a body in another thread
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    fn send_with_body(
        req: RequestBuilder<WithBody>,
        body: Option<serde_json::Value>,
    ) -> Result<Response<Body>, ureq::Error> {
        match body {
            None => req.send_empty(),
            Some(body) => req.send_json(body),
        }
    }
}

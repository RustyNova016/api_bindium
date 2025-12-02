use ureq::Body;
use ureq::RequestBuilder;
use ureq::http::Response;
use ureq::typestate::WithBody;
use ureq::typestate::WithoutBody;

use crate::ApiRequest;

impl<T> ApiRequest<T> {
    /// Send a request that doesn't require a body (ex: GET, DELETE)
    pub(super) fn send_without_body(
        req: RequestBuilder<WithoutBody>,
    ) -> Result<Response<Body>, ureq::Error> {
        req.config().http_status_as_error(false).build().call()
    }

    /// Send a request that require a body (ex: POST, PUT, PATCH)
    pub(super) fn send_with_body(
        req: RequestBuilder<WithBody>,
        body: Option<serde_json::Value>,
    ) -> Result<Response<Body>, ureq::Error> {
        let req = req.config().http_status_as_error(false).build();

        match body {
            Some(body) => req.send_json(body),
            None => req.send_empty(),
        }
    }
}

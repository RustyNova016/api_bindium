use std::collections::HashMap;

use crate::ApiRequest;
use crate::api_request::parsers::json::JsonParser;
use crate::endpoints::EndpointUriBuilder;

fn httpbin_get_request(arg: &str, value: &str) -> ApiRequest<JsonParser<HttpBinGetResponse>> {
    EndpointUriBuilder::new()
        .https()
        .set_authority("httpbin.org")
        .set_path("/get")
        .add_parameter(arg, value)
        .into_api_request(crate::HTTPVerb::Get)
        .unwrap()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct HttpBinGetResponse {
    args: HashMap<String, String>,
}

#[test]
#[cfg(feature = "sync")]
fn test_get_query() {
    use crate::ApiClient;

    let client = ApiClient::builder().build();
    let res = httpbin_get_request("hello", "world").send(&client).unwrap();

    assert_eq!(res.args.get("hello"), Some(&"world".to_string()))
}

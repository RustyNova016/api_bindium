use std::collections::HashMap;

use api_bindium::ApiClient;
use api_bindium::ApiRequest;
use api_bindium::HTTPVerb;
use api_bindium::api_request::parsers::json::JsonParser;
use api_bindium::endpoints::EndpointUriBuilder;

fn httpbin_get_request(arg: &str, value: &str) -> ApiRequest<JsonParser<HttpBinGetResponse>> {
    EndpointUriBuilder::new()
        .https()
        .set_authority("httpbin.org")
        .set_path("/get")
        .add_parameter(arg, value)
        .into_api_request(HTTPVerb::Get)
        .unwrap()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct HttpBinGetResponse {
    args: HashMap<String, String>,
}

fn main() {
    let client = ApiClient::builder().build();
    let res = httpbin_get_request("hello", "world").send(&client).unwrap();

    assert_eq!(res.args.get("hello"), Some(&"world".to_string()))
}

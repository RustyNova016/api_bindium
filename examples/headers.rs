use std::collections::HashMap;

use api_bindium::ApiClient;
use api_bindium::ApiRequest;
use api_bindium::HTTPVerb;
use api_bindium::api_request::parsers::json::JsonParser;
use api_bindium::endpoints::EndpointUriBuilder;

fn httpbin_get_request(arg: &str, value: &str) -> ApiRequest<JsonParser<HttpBinGetResponse>> {
    let uri = EndpointUriBuilder::new()
        .https()
        .set_authority("httpbin.org")
        .set_path("/get")
        .add_parameter(arg, value)
        .to_uri()
        .unwrap();

    let mut headers = HashMap::new();
    headers.insert("REFERER".to_string(), "github.com".to_string());

    ApiRequest::builder()
        .headers(headers)
        .uri(uri)
        .verb(HTTPVerb::Get)
        .build()
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

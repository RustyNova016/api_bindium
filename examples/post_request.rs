use api_bindium::ApiClient;
use api_bindium::ApiRequest;
use api_bindium::HTTPVerb;
use api_bindium::api_request::parsers::json::JsonParser;
use api_bindium::endpoints::EndpointUriBuilder;
use serde_json::json;

fn httpbin_post_request() -> ApiRequest<JsonParser<HttpBinPostResponse>> {
    EndpointUriBuilder::new()
        .https()
        .set_authority("httpbin.org")
        .set_path("/post")
        .into_api_request_with_body(
            HTTPVerb::Post,
            json!({
                "hello": "world"
            }),
        )
        .unwrap()
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct HttpBinPostResponse {
    data: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct HttpBinPostResponseData {
    hello: String,
}

fn main() {
    let client = ApiClient::builder().build();
    let res = httpbin_post_request().send(&client).unwrap();

    // HTTP bin send the body as a string instead of json... So deserializing required
    let json: HttpBinPostResponseData = serde_json::from_str(&res.data).unwrap();

    assert_eq!(json.hello, "world".to_string())
}

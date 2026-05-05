// This exemple shows how to create an endpoint test
//
// This is reusing code from the `post_request` exemple, so you probably want to see it before

use api_bindium::ApiClient;
use api_bindium::ApiRequest;
use api_bindium::HTTPVerb;
use api_bindium::JsonParser;
use api_bindium::endpoints::EndpointUriBuilder;
use api_bindium::parsers::json::testing::TestingJsonParser;
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
            JsonParser::default(),
        )
        .unwrap()
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct HttpBinPostResponse {
    data: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct HttpBinPostResponseData {
    hello: String,
}

// Since it's an exemple file we use `main()` here... But just think of it as your test function
fn main() {
    let client = ApiClient::builder().build();
    let res = httpbin_post_request()
        .assert_url("https://httpbin.org/post") // Check the URL. Will panic if incorrect
        .set_parser(TestingJsonParser::<HttpBinPostResponse>::default()) // We replace the parser by its testing equivalent
        .send(&client)
        .unwrap()
        .parse() // The endpoint is tested upon parse. It will direcly panic
        .unwrap();

    // Now you have the returned value. You can do some additional check to make sure the API sent the correct data.
    // But `api_bindium` can't help you here.

    // HTTP bin send the body as a string instead of json... So deserializing required
    let json: HttpBinPostResponseData = serde_json::from_str(&res.data).unwrap();

    assert_eq!(json.hello, "world".to_string())
}

use std::collections::HashMap;

use api_bindium::ApiClient;
use api_bindium::HTTPVerb;
use api_bindium::JsonParser;
use api_bindium::Parser;
use api_bindium::api_response::ureq_response::UreqResponseInner;
use api_bindium::endpoints::EndpointUriBuilder;
use serde_json::json;

// This exemple is a follow up to the `post_request` exemple.
// We'll create a parser for HTTPBin's /anything endpoint so we don't have to redeserialize our form data again
// For an exemple query, run `curl -X POST https://httpbin.org/anything --data '{"username":"xyz","password":"xyz"}'`

/// The json response from HTTPBin
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct HttpBinPostResponse {
    data: String,
}

/// This is our parser. It can hold some config if needed. We don't, so it's an empty struct.
pub struct HTTPBinParser;

// The parser trait is generic on the Client's response.
// In this case, the ureq client gives an UreqResponse<Parser>.
// But this poses errors down the line, so we need to use the special
// `UreqResponseInner` struct
impl Parser<UreqResponseInner> for HTTPBinParser {
    type Output = HashMap<String, String>;
    type Error = serde_json::Error;

    fn parse(&self, response: UreqResponseInner) -> Result<Self::Output, Self::Error> {
        // While we could a raw implementation of the parser, we can also use other parsers to do the work for us.
        // In this case, we deserialize the response to json directly
        let httpbin_response: HttpBinPostResponse = JsonParser::default().parse(response).unwrap();

        // We got the response, now let deserialize the inner data
        let inner_data: HashMap<String, String> = serde_json::from_str(&httpbin_response.data)?;

        Ok(inner_data)
    }
}

fn main() {
    let client = ApiClient::builder().build();
    let mut request = EndpointUriBuilder::new()
        .https()
        .set_authority("httpbin.org")
        .set_path("/post")
        .into_api_request_with_body(
            HTTPVerb::Post,
            json!({
                "hello": "world"
            }),
            HTTPBinParser,
        )
        .unwrap();

    let res = request.send(&client).unwrap().parse().unwrap();

    // Compared to the `post_request` exemple, we have nothing to do!

    assert_eq!(res.get("hello").unwrap(), "world");
}

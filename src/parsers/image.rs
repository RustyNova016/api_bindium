use image::DynamicImage;
use snafu::ResultExt;
use ureq::ResponseExt as _;

use crate::api_response::ureq_response::UreqResponseInner;
use crate::error::ImageParsingSnafu;
use crate::error::UreqSnafu;
use crate::parsers::Parser;

/// Parse the response into an [`image::DynamicImage`]
pub struct ImageParser;

impl Parser<UreqResponseInner> for ImageParser {
    type Output = DynamicImage;
    type Error = crate::ApiRequestError;

    fn parse(&self, mut response: UreqResponseInner) -> Result<Self::Output, Self::Error> {
        let bytes = response.data
            .body_mut()
            .with_config()
            .limit(response.max_body_size)
            .read_to_vec()
            .context(UreqSnafu {
                uri: response.data.get_uri().to_owned(),
            })?;

        image::load_from_memory(&bytes).context(ImageParsingSnafu { data: bytes })
    }
}

#[cfg(test)]
pub mod test {
    use crate::ApiClient;
    use crate::endpoints::EndpointUriBuilder;

    #[cfg(feature = "sync")]
    #[test]
    pub fn test_image_parsing() {
        use crate::parsers::image::ImageParser;

        let mut req: crate::ApiRequest<ImageParser> = EndpointUriBuilder::new()
            .https()
            .set_authority("avatars.githubusercontent.com")
            .set_path("/u/50844553")
            .add_parameter("v", 4)
            .into_api_request(crate::HTTPVerb::Get, ImageParser)
            .unwrap();

        let client = ApiClient::builder().build();
        req.send(&client).unwrap();
        req.reset();
    }
}

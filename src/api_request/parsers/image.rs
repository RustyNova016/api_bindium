use image::DynamicImage;
use snafu::ResultExt;
use ureq::ResponseExt as _;

use crate::api_request::error::ImageParsingSnafu;
use crate::api_request::error::UreqSnafu;
use crate::api_request::parsers::Parser;

pub struct ImageParser;

impl Parser<DynamicImage> for ImageParser {
    fn parse(
        response: &mut ureq::http::Response<ureq::Body>,
    ) -> Result<DynamicImage, crate::ApiRequestError> {
        let bytes = response.body_mut().read_to_vec().context(UreqSnafu {
            uri: response.get_uri().to_owned(),
        })?;

        image::load_from_memory(&bytes).context(ImageParsingSnafu { data: bytes })
    }
}

#[cfg(test)]
pub mod test {
    use crate::ApiClient;
    use crate::api_request::parsers::image::ImageParser;
    use crate::endpoints::EndpointUriBuilder;

    #[cfg(feature = "sync")]
    #[test]
    pub fn test_image_parsing() {
        let mut req: crate::ApiRequest<ImageParser> = EndpointUriBuilder::new()
            .https()
            .set_authority("avatars.githubusercontent.com")
            .set_path("/u/50844553")
            .add_parameter("v", 4)
            .into_api_request(crate::HTTPVerb::Get)
            .unwrap();

        let client = ApiClient::builder().build();
        req.send(&client).unwrap();
        req.reset();
    }
}

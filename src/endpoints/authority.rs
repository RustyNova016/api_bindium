use crate::endpoints::EndpointUriBuilder;
use crate::endpoints::path::EndpointUriBuilderPath;

pub struct EndpointUriBuilderAuthority;

impl EndpointUriBuilder<EndpointUriBuilderAuthority> {
    /// Set the authority of the uri
    pub fn set_authority(self, authority: &str) -> EndpointUriBuilder<EndpointUriBuilderPath> {
        EndpointUriBuilder {
            uri: format!("{}{authority}", self.uri),
            _state: EndpointUriBuilderPath,
        }
    }
}

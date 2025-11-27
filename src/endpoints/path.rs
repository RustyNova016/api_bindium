use crate::endpoints::EndpointUriBuilder;
use crate::endpoints::query::EndpointUriBuilderQuery;

pub struct EndpointUriBuilderPath;

impl EndpointUriBuilder<EndpointUriBuilderPath> {
    /// Set the path of the uri. The path must start with a leading `/`. For example: `https://httpcan.org/post` -> `/post`
    pub fn set_path(self, path: &str) -> EndpointUriBuilder<EndpointUriBuilderQuery> {
        EndpointUriBuilder {
            uri: format!("{}{path}", self.uri),
            _state: EndpointUriBuilderQuery,
        }
    }

    /// Validate the path and move on to the query parameters
    pub fn query(self) -> EndpointUriBuilder<EndpointUriBuilderQuery> {
        EndpointUriBuilder {
            uri: self.uri,
            _state: EndpointUriBuilderQuery,
        }
    }
}

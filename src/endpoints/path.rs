use crate::endpoints::EndpointUriBuilder;
use crate::endpoints::query::EndpointUriBuilderQuery;

pub struct EndpointUriBuilderPath;

impl EndpointUriBuilder<EndpointUriBuilderPath> {
    /// Set the path of the uri. The path must start with a leading `/`. For example: `https://httpcan.org/post` -> `/post`
    pub fn set_path(self, path: &str) -> EndpointUriBuilder<EndpointUriBuilderQuery> {
        self.add_path_fragment(path).query()
    }

    /// Add a path fragment to the current uri
    ///
    /// # Exemple:
    /// ```
    /// use api_bindium::endpoints::EndpointUriBuilder;
    ///
    /// let uri = EndpointUriBuilder::new().https().set_authority("api.org");
    /// let uri = uri.add_path_fragment("one_fragment");
    /// let uri = uri.add_path_fragment("multiple/fragments");
    ///
    /// assert_eq!(&uri.to_string(), "https://api.org/one_fragment/multiple/fragments")
    /// ```
    pub fn add_path_fragment(mut self, path: &str) -> Self {
        match (self.uri.ends_with('/'), path.starts_with('/')) {
            (true, true) => {
                self.uri.pop();
            }
            (false, false) => {
                self.uri.push('/');
            }
            _ => {}
        }

        EndpointUriBuilder {
            uri: format!("{}{path}", self.uri),
            _state: self._state,
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

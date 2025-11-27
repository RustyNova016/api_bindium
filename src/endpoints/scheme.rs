use crate::endpoints::EndpointUriBuilder;
use crate::endpoints::authority::EndpointUriBuilderAuthority;

pub struct EndpointUriBuilderScheme;

impl EndpointUriBuilder<EndpointUriBuilderScheme> {
    pub fn new() -> EndpointUriBuilder<EndpointUriBuilderScheme> {
        EndpointUriBuilder {
            _state: EndpointUriBuilderScheme,
            uri: String::new(),
        }
    }

    /// Set the scheme
    pub fn set_scheme(self, scheme: &str) -> EndpointUriBuilder<EndpointUriBuilderAuthority> {
        EndpointUriBuilder {
            uri: format!("{scheme}://"),
            _state: EndpointUriBuilderAuthority,
        }
    }

    /// Set the scheme to `http`
    pub fn http(self) -> EndpointUriBuilder<EndpointUriBuilderAuthority> {
        self.set_scheme("http")
    }

    /// Set the scheme to `https`
    pub fn https(self) -> EndpointUriBuilder<EndpointUriBuilderAuthority> {
        self.set_scheme("https")
    }
}

use core::fmt::Display;

use crate::endpoints::EndpointUriBuilder;

pub struct EndpointUriBuilderQuery;

impl EndpointUriBuilder<EndpointUriBuilderQuery> {
    /// Add a parameter to the uri
    pub fn add_parameter(
        self,
        name: &str,
        value: impl Display,
    ) -> EndpointUriBuilder<EndpointUriBuilderQuery> {
        let mut uri = self.uri;

        // Add an ? or & if needed
        if !uri.contains('?') {
            uri.push('?');
        } else {
            uri.push('&');
        }

        EndpointUriBuilder {
            uri: format!("{uri}{name}={value}"),
            _state: EndpointUriBuilderQuery,
        }
    }

    /// Add a parameter to the uri if the value is `Some`
    pub fn maybe_add_parameter(
        self,
        name: &str,
        value: Option<impl Display>,
    ) -> EndpointUriBuilder<EndpointUriBuilderQuery> {
        if let Some(value) = value {
            self.add_parameter(name, value)
        } else {
            self
        }
    }
}

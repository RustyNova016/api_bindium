use core::fmt::Display;

use url_escape::encode_special_query;

use crate::endpoints::EndpointUriBuilder;

pub struct EndpointUriBuilderQuery;

impl EndpointUriBuilder<EndpointUriBuilderQuery> {
    /// Add a parameter to the uri.
    ///
    /// The name and value are automatically escaped
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

        let name = encode_special_query(name);
        let value = value.to_string();
        let value = encode_special_query(&value);

        EndpointUriBuilder {
            uri: format!("{uri}{name}={value}"),
            _state: EndpointUriBuilderQuery,
        }
    }

    /// Add a parameter to the uri if the value is `Some`
    ///
    /// The name and value are automatically escaped
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

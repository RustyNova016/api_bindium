use core::fmt::Display;

/// The HTTP verb of the api request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HTTPVerb {
    Get,
    Post,
}

impl Display for HTTPVerb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HTTPVerb::Get => write!(f, "GET"),
            HTTPVerb::Post => write!(f, "POST"),
        }
    }
}

use core::error::Error as _;

use snafu::Snafu;
use ureq::http::Uri;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(super)))]
pub enum ApiRequestError {
    #[snafu(display("Couldn't successfully send the http request to {uri}"))]
    UreqError {
        source: ureq::Error,

        #[snafu(implicit)]
        location: snafu::Location,

        uri: Uri,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    #[snafu(display(
        "The max retry count for the request as been exeeded. You may want to check if the correct url is set, the server is online, or you aren't hitting the ratelimit."
    ))]
    MaxRetriesExceeded {
        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },

    #[snafu(display("The api's response couldn't be deserialized:\n{data}"))]
    JsonParsingError {
        source: serde_json::Error,
        data: String,

        #[snafu(implicit)]
        location: snafu::Location,

        #[cfg(feature = "backtrace")]
        backtrace: snafu::Backtrace,
    },
}

impl ApiRequestError {
    /// Return true if the error is temporary and should be retried
    pub fn is_retryable(&self) -> bool {
        self.is_timeout() || self.is_connection_reset()
    }

    /// Return true if the error is a timeout
    pub fn is_timeout(&self) -> bool {
        // Reqwest error
        let Some(source) = self.source() else {
            return false;
        };

        let Some(ureq_error) = source.downcast_ref::<ureq::Error>() else {
            return false;
        };

        matches!(ureq_error, ureq::Error::Timeout(_))
    }

    /// Return true if the error is a connection reset
    pub fn is_connection_reset(&self) -> bool {
        let Some(source) = self.source() else {
            return false;
        };

        let Some(ureq_error) = source.downcast_ref::<ureq::Error>() else {
            return false;
        };

        let ureq::Error::Io(std_error) = ureq_error else {
            return false;
        };

        std_error.kind() == std::io::ErrorKind::ConnectionReset
    }
}

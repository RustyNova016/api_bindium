use std::sync::Arc;

use ureq::Body;
use ureq::http::Response;

use crate::Parser;

/// A wrapper arround a [`ureq::Response<Body>`]
///
/// This also have some settings to read the response that are transfered from the [crate::ApiRequest]
#[derive(Debug)]
pub struct UreqResponseInner {
    /// The data of the response
    pub data: Response<Body>,

    /// The maximum size of the body of the response. This allows limiting response that may use more memories than it should
    pub max_body_size: u64,
}

/// A wrapper arround a [`UreqResponseInner`].
///
/// This only additionally holds the parser as the [`crate::Parser`] trait can't have a specified parser trait (`Parser<UreqResponseInner>` is fine, `Parser<UreqResponse<MyParser>>` will create compilations issues when used)
#[derive(Debug)]
pub struct UreqResponse<P> {
    /// The ureq response without the parser
    pub inner: UreqResponseInner,

    /// The parser to use on the response
    pub parser: Arc<P>,
}

impl<P> UreqResponse<P> {
    pub fn new(data: Response<Body>, max_body_size: u64, parser: Arc<P>) -> Self {
        Self {
            inner: UreqResponseInner {
                data,
                max_body_size,
            },
            parser,
        }
    }

    pub fn parse(
        self,
    ) -> Result<<P as Parser<UreqResponseInner>>::Output, <P as Parser<UreqResponseInner>>::Error>
    where
        P: Parser<UreqResponseInner>,
    {
        self.parser.clone().parse(self.inner)
    }
}

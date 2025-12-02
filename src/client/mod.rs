#[cfg(feature = "rate_limit")]
use core::num::NonZeroU32;
#[cfg(feature = "rate_limit")]
use std::sync::Arc;

#[cfg(feature = "rate_limit")]
use governor::Quota;
#[cfg(feature = "rate_limit")]
use governor::RateLimiter;
#[cfg(feature = "rate_limit")]
use governor::clock;
#[cfg(feature = "rate_limit")]
use governor::middleware::NoOpMiddleware;
#[cfg(feature = "rate_limit")]
use governor::state::InMemoryState;
#[cfg(feature = "rate_limit")]
use governor::state::NotKeyed;
use ureq::Agent;

/// The client handling the data for fetching
#[derive(Debug, bon::Builder, Clone)]
pub struct ApiClient {
    /// The [ureq::Agent] for the
    #[builder(default = ureq::agent())]
    pub agent: Agent,

    /// How many retries allowed before erroring out the request?
    #[builder(default = 10)]
    pub max_retries: u32,

    /// The inner ratelimiter
    #[cfg(feature = "rate_limit")]
    #[cfg_attr(feature = "rate_limit", builder(required, default = Some(default_ratelimit())))]
    pub rate_limit:
        Option<Arc<RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>>>,
}

impl ApiClient {
    /// Wait until the rate limit yield a spot
    #[cfg(feature = "rate_limit")]
    #[cfg_attr(feature = "hotpath", hotpath::measure)]
    pub async fn await_rate_limit(&self) {
        if let Some(rate) = &self.rate_limit {
            rate.until_ready().await
        }
    }
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::builder().build()
    }
}

#[cfg(feature = "rate_limit")]
fn default_ratelimit()
-> Arc<RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>> {
    let quota =
        Quota::per_second(NonZeroU32::new(1).unwrap()).allow_burst(NonZeroU32::new(5).unwrap());
    Arc::new(RateLimiter::direct(quota))
}

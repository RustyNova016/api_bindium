#[cfg(any(feature = "sync", feature = "async"))]
pub mod get;
#[cfg(feature = "sync")]
pub mod post;

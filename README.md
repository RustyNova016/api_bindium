# api_bindium

A lightweight crate to provide foundations for creating API binding crates. This aim to provide "batteries included" level of features while keeping itself lightweight

The async api is runtime agnostic (tokio isn't even in the tree!)

## Usage examples

You can find example crates at:
- [listenbrainz_rs](https://github.com/RustyNova016/listenbrainz_rs)

## Feature flags

Async: 
- `sync`: Enable the sync api
- `async`: Enable the async api (Sync and Async aren't mutually exclusive)

Fetching:
- `native_tls`: Use the system's native TLS. By default, Rustls is used to not have to depend on the system's tls
- `rustls` (default): Use rustls as tls provider.
- `rate_limit`: Add a rate limiter to the requests, using the `governor` crate. Please note that it only affect `async` variants of functions, as `governor` is made to work in async functions only. If you know a ratelimit crate that does both sync and async, feel free to submit an issue 

Parsing:
- `image`: Add an image parser using `image`

Debuging:
- `backtrace`: Enable error backtraces
- `tracing`: Enable tracing
- `hotpath`, `hotpath-alloc`, `hotpath-off`: Enable [hotpath](https://github.com/pawurb/hotpath-rs) debuging / perf analysis.

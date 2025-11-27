# api_maker

A crate to provide foundations for creating API binding crates.

## Feature flags

Async: 
- `sync`: Enable the sync api
- `async`: Enable the async api (Sync and Async aren't mutually exclusive)

Fetching:
- `native_tls`: Use the system's native TLS. By default, Rustls is used to not have to depend on the system's tls
- `rate_limit`: Add a rate limiter to the requests, using the `governor` crate. Please note that it only affect `async` variants of functions, as `governor` is made to work in async functions only. If you know a ratelimit crate that does both sync and async, feel free to submit an issue 

Debuging:
- `backtrace`: Enable error backtraces
- `tracing`: Enable tracing
- `hotpath`, `hotpath-alloc`, `hotpath-off`: Enable [hotpath](https://github.com/pawurb/hotpath-rs) debuging / perf analysis.

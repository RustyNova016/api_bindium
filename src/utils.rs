#[cfg(any(feature = "sync", feature = "async"))]
use std::time::Instant;

#[cfg(feature = "sync")]
pub(crate) fn sleep_until(until: Instant) {
    let dur = Instant::now() - until;
    if dur.as_secs() > 0 {
        use std::thread::sleep;

        sleep(dur);
    }
}

#[cfg(feature = "async")]
pub(crate) async fn sleep_until_async(until: Instant) {
    use async_io::Timer;

    Timer::at(until).await;
}

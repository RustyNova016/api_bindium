#[cfg(any(feature = "sync", feature = "async"))]
use std::time::Instant;

#[cfg(feature = "sync")]
pub(crate) fn sleep_until(until: Instant) {
    use std::thread::sleep;

    let dur = until - Instant::now();
    sleep(dur);
}

#[cfg(feature = "async")]
pub(crate) async fn sleep_until_async(until: Instant) {
    use async_io::Timer;

    Timer::at(until).await;
}

#[cfg(test)]
mod test {
    #[cfg(feature = "async")]
    use macro_rules_attribute::apply;

    #[cfg(feature = "sync")]
    #[test]
    fn test_sleep_until() {
        use core::time::Duration;
        use std::time::Instant;

        use crate::utils::sleep_until;

        let now = Instant::now();
        let then = now + Duration::from_secs(1);
        sleep_until(then);
        let now = Instant::now();
        assert!(now > then);
    }

    #[cfg(feature = "async")]
    #[apply(smol_macros::test!)]
    async fn test_sleep_until_async() {
        use core::time::Duration;
        use std::time::Instant;

        use crate::utils::sleep_until_async;

        let now = Instant::now();
        let then = now + Duration::from_secs(1);
        sleep_until_async(then).await;
        let now = Instant::now();
        assert!(now > then);
    }
}

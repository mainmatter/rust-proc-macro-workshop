use retry_exercise::retry;
use std::sync::atomic::{AtomicU32, Ordering};

static FLAKY_CALLS: AtomicU32 = AtomicU32::new(0);

// Fails the first two times, then succeeds. With up to 5 attempts it should reach
// the success on the third call.
#[retry(times = 5, delay_ms = 0)]
fn flaky() -> Result<u32, String> {
    let n = FLAKY_CALLS.fetch_add(1, Ordering::SeqCst) + 1;
    if n < 3 {
        Err(format!("attempt {n} failed"))
    } else {
        Ok(n)
    }
}

static GIVE_UP_CALLS: AtomicU32 = AtomicU32::new(0);

// Always fails: the macro should give up after exactly `times` attempts and return
// the final error.
#[retry(times = 2, delay_ms = 0)]
fn always_fails() -> Result<u32, String> {
    GIVE_UP_CALLS.fetch_add(1, Ordering::SeqCst);
    Err("nope".to_string())
}

static QUICK_CALLS: AtomicU32 = AtomicU32::new(0);

// `delay_ms` is omitted, so it falls back to its default of 0.
#[retry(times = 3)]
fn quick() -> Result<u32, String> {
    let n = QUICK_CALLS.fetch_add(1, Ordering::SeqCst) + 1;
    if n < 2 { Err("once".to_string()) } else { Ok(n) }
}

fn main() {
    // Retried until it succeeded, and stopped as soon as it did (3 calls, not 5).
    assert_eq!(flaky(), Ok(3));
    assert_eq!(FLAKY_CALLS.load(Ordering::SeqCst), 3);

    // Retried `times` times, then surfaced the last error.
    assert_eq!(always_fails(), Err("nope".to_string()));
    assert_eq!(GIVE_UP_CALLS.load(Ordering::SeqCst), 2);

    // `#[retry(times = 3)]` with no `delay_ms` uses the default and still retries.
    assert_eq!(quick(), Ok(2));
    assert_eq!(QUICK_CALLS.load(Ordering::SeqCst), 2);
}

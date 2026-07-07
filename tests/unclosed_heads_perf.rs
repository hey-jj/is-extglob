use std::time::{Duration, Instant};

use is_extglob::is_extglob;

#[test]
fn repeated_unclosed_heads_finish_quickly() {
    let input = "@(".repeat(32_000);

    let start = Instant::now();
    assert!(!is_extglob(&input));
    let elapsed = start.elapsed();

    assert!(
        elapsed < Duration::from_millis(250),
        "repeated unclosed heads took {elapsed:?}"
    );
}

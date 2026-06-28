//! Property tests comparing the scanner against a regex oracle.
//!
//! The oracle encodes the same rule with the regex `(\\).|([@?!+*]\(.*\))` and
//! a scan loop. The `.` in the regex crate excludes newline by default, which
//! is the behavior the scanner mirrors, so no `s` flag is set. Regex match
//! boundaries land on `char` boundaries, so slicing by the byte offset is
//! sound.

use is_extglob::is_extglob;
use proptest::prelude::*;
use regex::Regex;

/// An independent regex encoding of the same rule. Group 2 is the extglob; if
/// it participated the input has one. Otherwise skip past the escape and
/// rescan.
fn oracle(mut s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let re = Regex::new(r"(\\).|([@?!+*]\(.*\))").unwrap();
    while let Some(caps) = re.captures(s) {
        if caps.get(2).is_some() {
            return true;
        }
        let m = caps.get(0).unwrap();
        s = &s[m.end()..];
    }
    false
}

proptest! {
    /// The scanner agrees with the oracle over a generator biased toward
    /// extglob-shaped characters.
    #[test]
    fn agrees_with_oracle(s in r"[@?!+*()\\/|abcxyz \n]{0,12}") {
        prop_assert_eq!(is_extglob(&s), oracle(&s), "input: {:?}", s);
    }

    /// The scanner agrees with the oracle over arbitrary strings, including
    /// multibyte and other Unicode.
    #[test]
    fn agrees_with_oracle_any(s in ".{0,16}") {
        prop_assert_eq!(is_extglob(&s), oracle(&s), "input: {:?}", s);
    }

    /// Escaping the trigger of a real extglob neutralizes that occurrence.
    #[test]
    fn escaped_head_is_false(head in "[@?!+*]") {
        let escaped = format!("\\{head}(abc)");
        prop_assert!(!is_extglob(&escaped), "input: {:?}", escaped);
    }
}

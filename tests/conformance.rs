//! Conformance suite for `is_extglob`.
//!
//! Each entry is an `(input, expected)` pair grouped by the behavior it pins
//! down. The non-string inputs a dynamic language can pass (no argument, null,
//! arrays) have no `&str` representation, so they are omitted. The empty-string
//! case stays because it is reachable and meaningful.

use is_extglob::is_extglob;

/// Unescaped extglob heads return true.
const TRUE_UNESCAPED: &[&str] = &[
    "?(abc)",
    "@(abc)",
    "!(abc)",
    "*(abc)",
    "+(abc)",
    "xyz/?(abc)/xyz",
    "xyz/@(abc)/xyz",
    "xyz/!(abc)/xyz",
    "xyz/*(abc)/xyz",
    "xyz/+(abc)/xyz",
    "?(abc|xyz)/xyz",
    "@(abc|xyz)",
    "!(abc|xyz)",
    "*(abc|xyz)",
    "+(abc|xyz)",
];

/// Escaped or incomplete heads return false.
///
/// One backslash in a Rust literal is `\\`. The byte length of `"\\?(abc)"`
/// is 7, which guards against accidentally doubling the escape.
const FALSE_ESCAPED: &[&str] = &[
    // unclosed: a head with no later `)`
    "?(abc/xyz",
    "@(abc",
    "!(abc",
    "*(abc",
    "+(abc",
    "(a|b",
    // backslash before the trigger
    "\\?(abc)",
    "\\@(abc)",
    "\\!(abc)",
    "\\*(abc)",
    "\\+(abc)",
    "xyz/\\?(abc)/xyz",
    "xyz/\\@(abc)/xyz",
    "xyz/\\!(abc)/xyz",
    "xyz/\\*(abc)/xyz",
    "xyz/\\+(abc)/xyz",
    "\\?(abc|xyz)/xyz",
    "\\@(abc|xyz)",
    "\\!(abc|xyz)",
    "\\*(abc|xyz)",
    "\\+(abc|xyz)",
    // backslash before the paren
    "?\\(abc)",
    "@\\(abc)",
    "!\\(abc)",
    "*\\(abc)",
    "+\\(abc)",
    "xyz/?\\(abc)/xyz",
    "xyz/@\\(abc)/xyz",
    "xyz/!\\(abc)/xyz",
    "xyz/*\\(abc)/xyz",
    "xyz/+\\(abc)/xyz",
    "?\\(abc|xyz)/xyz",
    "@\\(abc|xyz)",
    "!\\(abc|xyz)",
    "*\\(abc|xyz)",
    "+\\(abc|xyz)",
];

/// An escaped extglob followed by a real one returns true.
const TRUE_MIXED: &[&str] = &[
    "\\?(abc)/?(abc)",
    "\\@(abc)/@(abc)",
    "\\!(abc)/!(abc)",
    "\\*(abc)/*(abc)",
    "\\+(abc)/+(abc)",
    "xyz/\\?(abc)/xyz/xyz/?(abc)/xyz",
    "xyz/\\@(abc)/xyz/xyz/@(abc)/xyz",
    "xyz/\\!(abc)/xyz/xyz/!(abc)/xyz",
    "xyz/\\*(abc)/xyz/xyz/*(abc)/xyz",
    "xyz/\\+(abc)/xyz/xyz/+(abc)/xyz",
    "\\?(abc|xyz)/xyz/?(abc|xyz)/xyz",
    "\\@(abc|xyz)/@(abc|xyz)",
    "\\!(abc|xyz)/!(abc|xyz)",
    "\\*(abc|xyz)/*(abc|xyz)",
    "\\+(abc|xyz)/+(abc|xyz)",
];

/// Strings with no unescaped extglob return false.
const FALSE_NOT_EXTGLOB: &[&str] = &[
    "",
    "? (abc)",
    "@.(abc)",
    "!&(abc)",
    "*z(abc)",
    "+~(abc)",
    "*.js",
    "!*.js",
    "!foo",
    "!foo.js",
    "**/abc.js",
    "abc/*.js",
    "abc/{a,b}.js",
    "abc/{a..z}.js",
    "abc/{a..z..2}.js",
    "abc/(aaa|bbb).js",
    "abc/?.js",
    "?.js",
    "[abc].js",
    "[^abc].js",
    "a/b/c/[a-z].js",
    "[a-j]*[^c]b/c",
    ".",
    "aa",
    "abc.js",
    "abc/def/ghi.js",
];

#[test]
fn unescaped_extglobs_return_true() {
    for &input in TRUE_UNESCAPED {
        assert!(is_extglob(input), "is_extglob({input:?}) expected true");
    }
}

#[test]
fn escaped_or_unclosed_return_false() {
    for &input in FALSE_ESCAPED {
        assert!(!is_extglob(input), "is_extglob({input:?}) expected false");
    }
}

#[test]
fn escaped_then_real_returns_true() {
    for &input in TRUE_MIXED {
        assert!(is_extglob(input), "is_extglob({input:?}) expected true");
    }
}

#[test]
fn non_extglobs_return_false() {
    for &input in FALSE_NOT_EXTGLOB {
        assert!(!is_extglob(input), "is_extglob({input:?}) expected false");
    }
}

/// Guards the escaping caveat: a single backslash literal is one byte.
#[test]
fn backslash_literal_length() {
    assert_eq!("\\?(abc)".len(), 7);
}

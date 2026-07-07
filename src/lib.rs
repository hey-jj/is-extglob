//! Detect extglob patterns in a string.
//!
//! An extglob is one of bash's extended pattern operators: a trigger character
//! from `@ ? ! + *` immediately followed by a parenthesized group, like
//! `?(abc)` or `@(a|b)`. This crate answers one question: does a string contain
//! an unescaped extglob? It does not match or expand the pattern. Tools that do
//! the heavier glob work use a check like this first to skip strings that need
//! no further processing.
//!
//! A backslash neutralizes the character after it. So `\?(abc)` and `?\(abc)`
//! are not extglobs. An escaped extglob followed by a real one still returns
//! `true` because the scan skips the escape and keeps looking.
//!
//! ```
//! use is_extglob::is_extglob;
//!
//! assert!(is_extglob("?(abc)"));
//! assert!(is_extglob("xyz/@(a|b)/xyz"));
//! assert!(!is_extglob("\\?(abc)")); // escaped trigger
//! assert!(!is_extglob("*.js")); // plain glob, not an extglob
//! assert!(!is_extglob("")); // empty string
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

/// Returns `true` if `s` contains an unescaped extglob pattern.
///
/// An extglob is a trigger character (`@ ? ! + *`) immediately followed by `(`,
/// then any run of characters, then a closing `)`. The closing paren may sit
/// anywhere later in the string. A backslash before the trigger or before the
/// `(` escapes the construct and it does not count.
///
/// The empty string returns `false`.
///
/// # Examples
///
/// ```
/// use is_extglob::is_extglob;
///
/// assert!(is_extglob("+(abc|xyz)"));
/// assert!(is_extglob("\\?(abc)/?(abc)")); // escaped one skipped, real one found
/// assert!(!is_extglob("@\\(abc)")); // escaped paren
/// assert!(!is_extglob("abc/(aaa|bbb).js")); // group has no trigger char
/// ```
///
/// The argument is generic over `AsRef<str>`, so `&str`, `String`, `&String`,
/// and `Cow<str>` all work without an explicit `.as_ref()` at the call site.
pub fn is_extglob(s: impl AsRef<str>) -> bool {
    let s = s.as_ref();
    if s.is_empty() {
        return false;
    }

    let bytes = s.as_bytes();
    let mut i = 0;
    let mut saw_head = false;
    while i < bytes.len() {
        let b = bytes[i];

        if saw_head {
            if b == b')' {
                return true;
            }
            if b == b'\n' {
                saw_head = false;
            }
            i += 1;
            continue;
        }

        // A backslash consumes the next character. Skip both the backslash and
        // the whole escaped character, then keep scanning. If the backslash is
        // the last byte it escapes nothing, so stop.
        if b == b'\\' {
            match s[i + 1..].chars().next() {
                Some(c) => i += 1 + c.len_utf8(),
                None => return false,
            }
            continue;
        }

        // A trigger directly followed by `(` starts a pending extglob head.
        if matches!(b, b'@' | b'?' | b'!' | b'+' | b'*') && bytes.get(i + 1) == Some(&b'(') {
            saw_head = true;
            i += 2;
            continue;
        }

        i += 1;
    }

    false
}

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
pub fn is_extglob(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];

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

        // A trigger directly followed by `(` and a later `)` is an extglob.
        if matches!(b, b'@' | b'?' | b'!' | b'+' | b'*')
            && bytes.get(i + 1) == Some(&b'(')
            && has_close_paren(bytes, i + 2)
        {
            return true;
        }

        i += 1;
    }

    false
}

/// True if a `)` appears at or after `start`, without crossing a newline.
///
/// The pattern run stops at a newline, so a closing paren on a later line does
/// not complete the extglob.
fn has_close_paren(bytes: &[u8], start: usize) -> bool {
    for &b in &bytes[start..] {
        if b == b'\n' {
            return false;
        }
        if b == b')' {
            return true;
        }
    }
    false
}

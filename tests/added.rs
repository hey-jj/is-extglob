//! Edge cases the core suite does not cover.
//!
//! These lock down greedy matching, backslash handling at string ends, newline
//! behavior, and multibyte re-scan offsets.

use is_extglob::is_extglob;

/// `(input, expected)` for behaviors at the edges of the rule.
const CASES: &[(&str, bool)] = &[
    // empty body: any-run between parens may be empty
    ("@()", true),
    // greedy match still finds a `)` after the `(`
    ("@(foo)bar)", true),
    // `)` then `(` after the trigger: no group forms
    ("@)abc(", false),
    // trigger not followed by `(`
    ("@no paren", false),
    // lone backslash matches nothing and ends the scan
    ("\\", false),
    // trailing backslash is inert
    ("abc\\", false),
    // escaped backslash consumed first, leaving a real extglob
    ("\\\\?(abc)", true),
    // newline before the `)`: the run does not cross it
    ("@(a\nb)", false),
    // first head completes on its own line
    ("@(a)\n@(b)", true),
    // multibyte char before an escaped then real extglob
    ("é\\?(a)/?(a)", true),
    // both parens escaped: trigger never adjacent to a literal `(`
    ("@\\(abc\\)", false),
    // head with no closing paren
    ("@(abc", false),
    // 4-byte char inside the body: scan handles a non-BMP char in the group
    ("@(😀)", true),
    // 4-byte char before the trigger: the prefix does not slip the index
    ("😀@(a)", true),
    // escaped 4-byte char skipped, real extglob found after it
    ("\\😀@(a)", true),
    // trigger then escaped 4-byte char: paren not adjacent to the trigger
    ("@\\😀(a)", false),
    // the only `)` sits past the newline, so the head never closes on its line
    ("@(a\nb)c)", false),
    // head closes before the newline, so the later newline does not matter
    ("@(a)b\nc)", true),
];

#[test]
fn edge_cases_match_rule() {
    for &(input, expected) in CASES {
        assert_eq!(
            is_extglob(input),
            expected,
            "is_extglob({input:?}) expected {expected}"
        );
    }
}

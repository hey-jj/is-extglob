# is-extglob

Detect extglob patterns in a string.

An extglob is one of bash's extended pattern operators: a trigger character from
`@ ? ! + *` immediately followed by a parenthesized group, like `?(abc)` or
`@(a|b)`. This crate reports whether a string contains an unescaped extglob. It
does not match or expand the pattern. Glob tooling uses a check like this first
to skip strings that need no further work.

## Installation

```toml
[dependencies]
is-extglob = "0.1"
```

## Usage

```rust
use is_extglob::is_extglob;

assert!(is_extglob("?(abc)"));
assert!(is_extglob("xyz/@(a|b)/xyz"));
assert!(is_extglob("\\?(abc)/?(abc)")); // escaped one skipped, real one found

assert!(!is_extglob("\\?(abc)")); // escaped trigger
assert!(!is_extglob("@\\(abc)")); // escaped paren
assert!(!is_extglob("*.js")); // plain glob, not an extglob
assert!(!is_extglob("")); // empty string
```

## Behavior

- A trigger char directly followed by `(` and a later `)` is an extglob.
- A backslash neutralizes the next character, so `\?(abc)` and `?\(abc)` are not
  extglobs.
- An escaped extglob followed by a real one returns `true`.
- A closing paren on a later line does not complete the pattern.
- The empty string returns `false`.

No runtime dependencies.

## License

Licensed under the [MIT license](LICENSE).

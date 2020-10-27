# Count Token Trees Macro for Rust

[![crates.io](https://img.shields.io/crates/v/count_tts.svg)](https://crates.io/crates/count_tts)
[![Docs](https://docs.rs/count_tts/badge.svg)](https://docs.rs/count_tts/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A simple macro for counting tokens.

```rust
use count_tts::count_tts;

assert_eq!(
    count_tts!(1 2 3 4 5 6 7),
    7
);
```

### `macro_rules!` alternative

If you prefer to avoid adding a dependency, here is an equivalent
`macro_rules!` definition that you may copy into your code.

```rust
/// `macro_rules!` implementation of `count_tts`.
/// Source: https://github.com/camsteffen/count-tts
macro_rules! count_tts {
    () => (0);
    ($one:tt) => (1);
    ($($a:tt $b:tt)+) => (count_tts!($($a)+) << 1);
    ($odd:tt $($a:tt $b:tt)+) => (count_tts!($($a)+) << 1 | 1);
}
```

This is slightly less optimal than the macro provided by the crate since it is
not a procedural macro. Instead of just a number, it generates an equivalent
expression of `1`'s and bitwise operators. It is optimized for logarithmic
recursion and output length.

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

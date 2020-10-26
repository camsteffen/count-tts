# Count Token Trees Macro for Rust

[![crates.io](https://img.shields.io/crates/v/count_tts.svg)](https://crates.io/crates/count_tts)
[![Docs](https://docs.rs/count_tts/badge.svg)](https://docs.rs/count_tts/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Description

A simple and optimized macro for counting tokens.

The macro uses bitshift operators and a logarithmic parsing algorithm to minimize output
length and parsing recursion.

```rust
use count_tts::count_tts;

assert_eq!(
    // expands to (1 << 1 | 1) << 1 | 1
    count_tts!(1 2 3 4 5 6 7),
    7
);
```

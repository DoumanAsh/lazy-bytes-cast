lazy-bytes-cast
==============

[![Build Status](https://travis-ci.org/DoumanAsh/lazy-bytes-cast.svg?branch=master)](https://travis-ci.org/DoumanAsh/lazy-bytes-cast)

This crate provides simple methods to cast from and into byte arrays.

# Note

The crates will not take care of byte order for you. Cuz lazy.

# Usage example

```rust
extern crate lazy_bytes_cast;

use lazy_bytes_cast::{
    ToBytesCast,
    FromBytesCastLazy
};

fn main() {
    let int_to: u32 = u32::max_value();
    println!("result={:?}", int_to.to_bytes());

    let bytes: [u8; 4] = [255, 255, 255, 255];
    let result: u32 = bytes.cast_to();
    println!("result={}", result);
}
```


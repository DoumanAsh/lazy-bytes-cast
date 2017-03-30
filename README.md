# lazy-bytes-cast

[![Build status](https://ci.appveyor.com/api/projects/status/hox713p4a6enyfym/branch/master?svg=true)](https://ci.appveyor.com/project/DoumanAsh/lazy-bytes-cast/branch/master)
[![Build Status](https://travis-ci.org/DoumanAsh/lazy-bytes-cast.svg?branch=master)](https://travis-ci.org/DoumanAsh/lazy-bytes-cast)
[![Crates.io](https://img.shields.io/crates/v/lazy-bytes-cast.svg)](https://crates.io/crates/lazy-bytes-cast)
[![Docs.rs](https://docs.rs/lazy-bytes-cast/badge.svg)](https://docs.rs/crate/lazy-bytes-cast/)

This crate provides simple methods to cast from and into byte arrays.

## Note

The crates will not take care of byte order for you. Cuz lazy.


## Usage

### Slice integer as bytes

```rust
extern crate lazy_bytes_cast;
use lazy_bytes_cast::slice::{
   ByteSlice,
   ByteIndex
};

fn main() {
    let some_int = 666;

    for (idx, byte) in some_int.byte_slice().iter().enumerate() {
        assert_eq!(some_int.byte(idx).unwrap(), *byte);
        println!("bytes[{}]={}", idx, byte);
    }
}
```

### Cast bytes to integer

```rust
extern crate lazy_bytes_cast;
use lazy_bytes_cast::from::{
   bytes_cast
};

fn main() {
    let bytes = vec![127, 150, 152, 0];

    if let Some(int) = bytes_cast::<u32>(&bytes) {
        println!("bytes={}", int);
    }
    else {
        println!("Couldn't extract integer from bytes");
    }
}
```

### Cast integer to bytes

```rust
extern crate lazy_bytes_cast;
use lazy_bytes_cast::to::{
   ToBytesCast
};

fn main() {
    let some_int = 666;

    let bytes = some_int.to_bytes();

    println!("bytes={:?}", bytes);
}
```


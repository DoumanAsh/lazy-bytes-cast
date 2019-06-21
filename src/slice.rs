//!Slice accessors
//!
//!Provides traits to access integers as byte slices.
//!
//!# Usage
//!
//!```rust
//!use lazy_bytes_cast::slice::{
//!   ByteSlice,
//!   ByteIndex
//!};
//!
//!fn main() {
//!    let some_int = 666;
//!
//!    for (idx, byte) in some_int.byte_slice().iter().enumerate() {
//!        assert_eq!(some_int.byte(idx).unwrap(), *byte);
//!        println!("bytes[{}]={}", idx, byte);
//!    }
//!}
//!```

use core::mem;
use core::slice as std_slice;

/// Slice Accessor.
///
/// Note that one must be careful when impl this trait for own types.
pub unsafe trait ByteSlice: Sized {
    /// Returns read-only slice over integer bytes
    fn byte_slice<'a>(&'a self) -> &'a [u8] {
        unsafe {
            std_slice::from_raw_parts(self as *const _ as *const _, mem::size_of::<Self>())
        }
    }
    /// Returns mutable slice over integer bytes
    fn byte_mut_slice<'a>(&'a mut self) -> &'a mut [u8] {
        unsafe {
            std_slice::from_raw_parts_mut(self as *mut _ as *mut _, mem::size_of::<Self>())
        }
    }
}

/// Indexing Accessor.
pub unsafe trait ByteIndex : ByteSlice {
    /// Returns byte from integer by index.
    ///
    ///# Parameters:
    ///
    ///* `idx` - Index of byte starting from 0.
    ///
    ///# Result:
    ///
    ///* `Some` - Contains byte.
    ///* `None` - Index out of bounds.
    fn byte(&self, idx: usize) -> Option<u8> {
        if idx >= mem::size_of::<Self>() {
            return None;
        }

        Some(self.byte_slice()[idx])
    }
}

macro_rules! impl_trait {
    ($($type:ty,)+) => {
        $(
            unsafe impl ByteSlice for $type {}
            unsafe impl ByteIndex for $type {}
        )+
    }
}

impl_trait!(u8, i8, u16, i16, u32, i32, f32, u64, i64, f64, usize, isize,);

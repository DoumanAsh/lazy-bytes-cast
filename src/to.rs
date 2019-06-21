//!Conversions to bytes.
//!
//!# Usage
//!
//!```rust
//!use lazy_bytes_cast::to::{ToBytesCast};
//!
//!fn main() {
//!    let some_int = 666;
//!
//!    let bytes = some_int.to_bytes();
//!
//!    println!("bytes={:?}", bytes);
//!}
//!```

use core::{marker, mem, ptr};
#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

///Converts data to a byte array.
///
///# Note:
///
///This function limits its usage to data that implements `marker::Copy`
///
///But it does not guarantee that all types with such trait will be correctly converted.
///
///# Parameters:
///
///* `data` - Arbitrary data that can be `memcpy`
///
///# Result:
///
///* `Vec` - Allocated with size equal to `size_of::<data>`.
pub fn bytes<T: marker::Copy>(data: T) -> Vec<u8> {
    let len = mem::size_of::<T>();
    let mut result: Vec<u8> = vec![0; len];

    unsafe {
        ptr::copy_nonoverlapping(&data as *const _ as *const u8, result.as_mut_ptr(), len);
    }

    result
}

///Unsafe version of [copy_bytes](fn.copy_bytes.html)
///
///Note: Unsafe version doesn't check anything and just copies data according to its size into slice.
pub unsafe fn copy_bytes_lazy<T: marker::Copy>(data: T, to: &mut [u8]) {
    ptr::copy_nonoverlapping(&data as *const _ as *const u8, to.as_mut_ptr(), mem::size_of::<T>());
}

///Converts data to a byte array by writing it into mutable slice
///
///# Note:
///
///This function limits its usage to data that implements `marker::Copy`
///
///But it does not guarantee that all types with such trait will be correctly converted.
///
///# Parameters:
///
///* `data` - Arbitrary data that can be `memcpy`
///* `to` - Byte slice where to copy
///
///# Result:
///
///* `Ok` - Success.
///* `Err` - Insufficient slice size.
pub fn copy_bytes<T: marker::Copy>(data: T, to: &mut [u8]) -> Result<(), ()> {
    let len = mem::size_of::<T>();

    if to.len() < len {
        return Err(());
    }

    unsafe {
        ptr::copy_nonoverlapping(&data as *const _ as *const u8, to.as_mut_ptr(), len);
    }

    Ok(())
}

///Trait to provide `to_bytes` method for a arbitrary data.
///
///This trait is implemented for a basic integer that can be safely converted.
pub unsafe trait ToBytesCast : marker::Copy {
    ///Converts to bytes.
    fn to_bytes(&self) -> Vec<u8>;
    ///Writes into byte slice.
    fn copy_to_bytes(&self, to: &mut [u8]) -> Result<(), ()>;
    ///Unsafe version of `copy_to_bytes`
    unsafe fn copy_to_bytes_lazy(&self, to: &mut [u8]);
}

macro_rules! impl_to_traits
{
    ($($t:ty), +) => {
        $(
            unsafe impl ToBytesCast for $t {
                #[inline]
                fn to_bytes(&self) -> Vec<u8> {
                    bytes(*self)
                }

                #[inline]
                fn copy_to_bytes(&self, to: &mut [u8]) -> Result<(), ()> {
                    copy_bytes(*self, to)
                }

                #[inline]
                unsafe fn copy_to_bytes_lazy(&self, to: &mut [u8]) {
                    copy_bytes_lazy(*self, to)
                }
            }
        )+
    };
}

impl_to_traits!(u64, u32, u16, u8, usize, i64, i32, i16, i8, isize, f32, f64, i128, u128);

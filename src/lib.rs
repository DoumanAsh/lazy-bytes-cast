/*!
This crate provides simple methods to cast from and into byte arrays.

# Note

The crates will not take care of byte order for you. Cuz lazy.

# Usage example

```rust
extern crate lazy_bytes_cast;

use lazy_bytes_cast::{
    ToBytesCast,
    FromBytesCast
};

fn main() {
    let int_to: u32 = u32::max_value();
    println!("result={:?}", int_to.to_bytes());

    let bytes: [u8; 4] = [255, 255, 255, 255];
    let result: u32 = bytes.cast_to().unwrap();
    println!("result={}",result);
}
```

*/

///Converts data to a byte array.
///
///# Note:
///
///This function limits its usage to data that implements ```marker::Copy```
///
///But it does not guarantee that all types with such trait will be correctly converted.
///
///# Parameters:
///
///* ```data``` - Arbitrary data that can be ```memcpy```
///
///# Result:
///
///* ```Vec``` - Allocated with size equal to ```size_of::<data>```.
pub fn to_bytes<T: std::marker::Copy>(data: T) -> Vec<u8> {
    let len = std::mem::size_of::<T>();
    let mut result: Vec<u8> = vec![0; len];

    unsafe {
        std::ptr::copy_nonoverlapping(&data as *const _ as *const u8, result.as_mut_ptr(), len);
    }

    result
}

///Trait to provide ```to_bytes``` method for a arbitrary data.
///
///This trait is implemented for a basic integer that can be safely converted.
pub unsafe trait ToBytesCast : std::marker::Copy {
    fn to_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_to_traits
{
    ($($t:ty), +) => {
        $(
            unsafe impl ToBytesCast for $t {
                #[inline]
                fn to_bytes(&self) -> Vec<u8> {
                    to_bytes(*self)
                }
            }
        )+
    };
}

impl_to_traits!(u64, u32, u16, u8, usize, i64, i32, i16, i8, isize, f32, f64);

///Unsafe version of ```bytes_cast```
///
///# Note:
///
///This function is able to convert only to types that implements ```marker::Copy```
///
///# Parameters:
///
///* ```bytes``` - slice of bytes to convert.
///
///# Result:
///
///* ```T``` - Converted data.
pub unsafe fn bytes_cast_lazy<T: std::marker::Copy>(bytes: &[u8]) -> T {
    let len = std::mem::size_of::<T>();

    let mut result: T = std::mem::uninitialized();
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), &mut result as *mut _ as *mut u8, len);

    result
}

///Converts slice of bytes to an integer.
///
///# Note:
///
///This function allows conversion only to types that implements ```ToBytesCast```.
///
///# Parameters:
///
///* ```bytes``` - slice of bytes to convert
///
///# Result:
///
///* ```Ok``` - Converted integer.
///* ```Err``` - Insufficient bytes size for a cast.
pub fn bytes_cast<T: ToBytesCast>(bytes: &[u8]) -> Result<T, String> {
    let len = std::mem::size_of::<T>();

    if bytes.len() < len {
        return Err("Bytes size is insufficient for a cast".to_string());
    }

    unsafe {
        Ok(bytes_cast_lazy(bytes))
    }
}

///Trait to provide casting function to byte slices
pub unsafe trait FromBytesCast<T: ToBytesCast> {
    fn cast_to(&self) -> Result<T, String>;
}

unsafe impl<T: ToBytesCast> FromBytesCast<T> for Vec<u8> {
    #[inline]
    fn cast_to(&self) -> Result<T, String> {
        bytes_cast(self)
    }
}

unsafe impl<T: ToBytesCast> FromBytesCast<T> for [u8] {
    #[inline]
    fn cast_to(&self) -> Result<T, String> {
        bytes_cast(self)
    }
}

unsafe impl<'a, T: ToBytesCast> FromBytesCast<T> for &'a[u8] {
    #[inline]
    fn cast_to(&self) -> Result<T, String> {
        bytes_cast(*self)
    }
}

macro_rules! impl_from_traits
{
    ($([$t:ty; $size:expr]), +) => {
        $(
            unsafe impl FromBytesCast<$t> for [u8; $size] {
                #[inline]
                fn cast_to(&self) -> Result<$t, String> {
                    unsafe {
                        Ok(bytes_cast_lazy(self))
                    }
                }
            }
        )+
    };
}

impl_from_traits!([u32; 4], [i32; 4], [f32; 4],
                  [u64; 8], [i64; 8], [f64; 8],
                  [u16; 2], [i16; 2]);


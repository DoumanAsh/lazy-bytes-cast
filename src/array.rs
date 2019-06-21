//!Conversion to static array
//!
//!# Usage
//!
//!```rust
//!use lazy_bytes_cast::array::IntoByteArray;
//!
//!fn main() {
//!    let array = 666.into_byte_array();
//!
//!    println!("bytes={:?}", array);
//!}
//!```

use core::mem;

///Describes conversion to byte array.
pub trait IntoByteArray: Copy {
    ///Type into which to convert.
    type Array: Copy;

    ///Performs conversion of self into `Array`.
    fn into_byte_array(self) -> Self::Array;
}

macro_rules! impl_trait {
    ($($type:ty,)+) => {
        $(
            impl IntoByteArray for $type {
                type Array = [u8; mem::size_of::<$type>()];

                fn into_byte_array(self) -> Self::Array {
                    unsafe { mem::transmute(self) }
                }
            }
        )+
    }
}

impl_trait!(u8, i8, u16, i16, u32, i32, f32, u64, i64, f64, usize, isize, u128, i128,);

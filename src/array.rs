//! Array conversion

use core::mem;

///Describes conversion to bytes representation
pub trait IntoBytes: Copy {
    ///Type into which to convert.
    type Array: Copy + AsRef<[u8]> + AsMut<[u8]> + core::borrow::BorrowMut<[u8]> + core::fmt::Debug;

    ///Performs conversion of self into `Array`.
    fn into_bytes(self) -> Self::Array;
}

///Describes conversion from bytes representation
pub unsafe trait FromBytes {
    ///Type into which to convert.
    type Array: Copy + AsRef<[u8]> + AsMut<[u8]> + core::borrow::BorrowMut<[u8]> + core::fmt::Debug;

    ///Converts array to self.
    fn from_bytes(arr: Self::Array) -> Self;
}

macro_rules! impl_trait {
    ($($type:ty,)+) => {
        $(
            impl IntoBytes for $type {
                type Array = [u8; mem::size_of::<$type>()];

                #[inline(always)]
                fn into_bytes(self) -> Self::Array {
                    self.to_ne_bytes()
                }
            }

            unsafe impl FromBytes for $type {
                type Array = [u8; mem::size_of::<$type>()];

                #[inline(always)]
                fn from_bytes(arr: Self::Array) -> Self {
                    Self::from_ne_bytes(arr)
                }
            }
        )+
    }
}

impl_trait!(u8, i8, u16, i16, u32, i32, f32, u64, i64, f64, usize, isize, u128, i128,);

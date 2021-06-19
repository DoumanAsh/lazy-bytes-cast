///Generic trait to convert to bytes.
///
///## Example
///```
///use lazy_bytes_cast::IntoBytes;
///
///pub fn to_array<T: IntoBytes<S>, const S: usize>(val: T) -> [u8; S] {
///    val.into_bytes()
///}
///
///assert_eq!(to_array(u16::max_value()), [255, 255]);
///```
pub trait IntoBytes<const T: usize> {
    ///Converts self into bytes
    fn into_bytes(self) -> [u8; T];
}

///Generic trait to convert from bytes.
///
///## Example
///```
///use lazy_bytes_cast::FromBytes;
///
///let result: u16 = FromBytes::from_bytes([255, 255]);
///assert_eq!(result, u16::max_value());
///```
pub trait FromBytes<const T: usize> {
    ///Converts into self from bytes
    fn from_bytes(arr: [u8; T]) -> Self;
}

macro_rules! impl_trait {
    ($($type:ty,)+) => {
        $(
            impl IntoBytes<{core::mem::size_of::<$type>()}> for $type {
                #[inline(always)]
                fn into_bytes(self) -> [u8; core::mem::size_of::<$type>()] {
                    self.to_ne_bytes()
                }
            }

            impl FromBytes<{core::mem::size_of::<$type>()}> for $type {
                #[inline(always)]
                fn from_bytes(arr: [u8; core::mem::size_of::<$type>()]) -> Self {
                    Self::from_ne_bytes(arr)
                }
            }
        )+
    }
}

impl_trait!(u8, i8, u16, i16, u32, i32, f32, u64, i64, f64, usize, isize, u128, i128,);

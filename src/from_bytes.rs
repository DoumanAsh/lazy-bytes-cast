use std::{
    marker,
    mem,
    ptr
};

use to_bytes::{
    ToBytesCast
};

///Unsafe version of `bytes_cast`
///
///# Note:
///
///This function is able to convert only to types that implements `marker::Copy`
///
///# Parameters:
///
///* `bytes` - slice of bytes to convert.
///
///# Result:
///
///* `T` - Converted data.
pub unsafe fn bytes_cast_lazy<T: marker::Copy>(bytes: &[u8]) -> T {
    let len = mem::size_of::<T>();

    let mut result: T = mem::uninitialized();
    ptr::copy_nonoverlapping(bytes.as_ptr(), &mut result as *mut _ as *mut u8, len);

    result
}

///Converts slice of bytes to an integer.
///
///# Note:
///
///To be safe this function allows conversion only to types that implements `ToBytesCast`.
///
///# Parameters:
///
///* `bytes` - slice of bytes to convert
///
///# Result:
///
///* `Ok` - Converted integer.
///* `Err` - Insufficient bytes size for a cast.
pub fn bytes_cast<T: ToBytesCast>(bytes: &[u8]) -> Result<T, ()> {
    let len = mem::size_of::<T>();

    if bytes.len() < len {
        return Err(());
    }

    unsafe {
        Ok(bytes_cast_lazy(bytes))
    }
}

///Provides casting from bytes slice.
///
///If you're abosolutely hate to bother yourself by checking Result.
///
///Feel free to just use `bytes_cast_lazy`
pub unsafe trait FromBytesCast<T: ToBytesCast> {
    fn cast_to(&self) -> Result<T, ()>;
}

unsafe impl<T: ToBytesCast> FromBytesCast<T> for Vec<u8> {
    #[inline]
    fn cast_to(&self) -> Result<T, ()> {
        bytes_cast(self)
    }
}

unsafe impl<T: ToBytesCast> FromBytesCast<T> for [u8] {
    #[inline]
    fn cast_to(&self) -> Result<T, ()> {
        bytes_cast(self)
    }
}

unsafe impl<'a, T: ToBytesCast> FromBytesCast<T> for &'a[u8] {
    #[inline]
    fn cast_to(&self) -> Result<T, ()> {
        bytes_cast(*self)
    }
}

///Provides casting from fixed size arrays to integers
///
///Supposed to be safe without wrapping into `Result`.
///
///Besides isn't it bothersome to unwrap perfectly safe cast from bytes array? ;)
pub unsafe trait FromBytesCastLazy<T: ToBytesCast> {
    #[inline]
    fn cast_to(&self) -> T;
}

macro_rules! impl_from_traits_arr
{
    ($([$t:ty; $size:expr]), +) => {
        $(
            unsafe impl FromBytesCastLazy<$t> for [u8; $size] {
                #[inline]
                fn cast_to(&self) -> $t {
                    unsafe {
                        bytes_cast_lazy(self)
                    }
                }
            }
        )+
    };
}

impl_from_traits_arr!(
    [u32; 4], [i32; 4], [f32; 4],
    [u64; 8], [i64; 8], [f64; 8],
    [u16; 2], [i16; 2]
);

#[cfg(target_pointer_width = "64")]
impl_from_traits_arr!([usize; 8], [isize; 8]);

#[cfg(target_pointer_width = "32")]
impl_from_traits_arr!([usize; 4], [isize; 4]);

macro_rules! impl_from_traits_tuple4
{
    ($($t:ty),+) => {
        $(
            unsafe impl FromBytesCastLazy<$t> for (u8, u8, u8, u8) {
                #[inline]
                fn cast_to(&self) -> $t {
                    [self.0, self.1, self.2, self.3].cast_to()
                }
            }
        )+
    };
}
macro_rules! impl_from_traits_tuple8
{
    ($($t:ty),+) => {
        $(
            unsafe impl FromBytesCastLazy<$t> for (u8, u8, u8, u8, u8, u8, u8, u8) {
                #[inline]
                fn cast_to(&self) -> $t {
                    [self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7].cast_to()
                }
            }
        )+
    };
}
macro_rules! impl_from_traits_tuple2
{
    ($($t:ty),+) => {
        $(
            unsafe impl FromBytesCastLazy<$t> for (u8, u8) {
                #[inline]
                fn cast_to(&self) -> $t {
                    [self.0, self.1].cast_to()
                }
            }
        )+
    };
}

impl_from_traits_tuple2!(i16, u16);
impl_from_traits_tuple4!(i32, u32, f32);
impl_from_traits_tuple8!(i64, u64, f64);

#[cfg(target_pointer_width = "64")]
impl_from_traits_tuple8!(isize, usize);

#[cfg(target_pointer_width = "32")]
impl_from_traits_tuple4!(isize, usize);

//!Array related utilities
use core::{mem, ptr, marker};

const unsafe fn transmute_slice<IN, OUT>(val: &IN) -> &[OUT] {
    &*ptr::slice_from_raw_parts(val as *const IN as *const OUT, mem::size_of::<IN>())
}

unsafe fn transmute_slice_mut<IN, OUT>(val: &mut IN) -> &mut [OUT] {
    &mut *ptr::slice_from_raw_parts_mut(val as *mut IN as *mut OUT, mem::size_of::<IN>())
}

const unsafe fn transmute_ref<IN, OUT: Copy>(val: &IN) -> OUT {
    *(mem::transmute::<_, &mem::MaybeUninit<OUT>>(val).assume_init_ref())
}

unsafe fn transmute_ref_unaligned<IN, OUT: Copy>(val: &IN) -> OUT {
    let mut out = mem::MaybeUninit::<OUT>::uninit();
    unsafe {
        ptr::copy_nonoverlapping(val as *const IN as *const u8, out.as_mut_ptr() as *mut u8, mem::size_of::<OUT>());
        out.assume_init()
    }
}

///Marker indicating that it is plain old data.
pub unsafe trait Pod: Copy {
}

macro_rules! impl_pod {
    ($($ty:ident),*) => {$(
        unsafe impl Pod for $ty {}
    )*};
}

impl_pod!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
unsafe impl<T: Pod> Pod for mem::ManuallyDrop<T> {}

struct Validator<IN, OUT> {
    _result: marker::PhantomData<(IN, OUT)>,
}

impl<IN, OUT: Copy> Validator<IN, OUT> {
    const IS_NOT_ZST: () = {
        assert!(mem::size_of::<IN>() > 0);
        assert!(mem::size_of::<OUT>() > 0);
    };

    const IS_SAME_SIZE: () = {
        assert!(mem::size_of::<IN>() == mem::size_of::<OUT>());
    };

    const IS_ENOUGH_BYTES: () = {
        assert!(mem::size_of::<IN>() >= mem::size_of::<OUT>());
    };
    const READ_FN: unsafe fn(&IN) -> OUT = {
        if mem::align_of::<OUT>() > mem::align_of::<IN>() {
            transmute_ref_unaligned
        } else {
            transmute_ref
        }
    };
}

///Gets uninit byte slice out of the type.
///
///Because type of any size can be safely represented as slice of uninitialized bytes, this
///method is safe to use.
///It is up to user to interpret result correctly and safe.
///
///## Usage
///
///```
///use lazy_bytes_cast::uninit_byte_slice_from;
///use core::mem::MaybeUninit;
///
///static BYTES: &[MaybeUninit<u8>] = uninit_byte_slice_from(&0u32);
///```
///
///## Restrictions
///
///Compilation fails for ZST.
///
///```compile_fail
///use lazy_bytes_cast::uninit_byte_slice_from;
///use core::mem::MaybeUninit;
///
///static BYTES: &[MaybeUninit<u8>] = uninit_byte_slice_from(&());
///```
pub const fn uninit_byte_slice_from<T>(val: &T) -> &[mem::MaybeUninit<u8>] {
    let _ = Validator::<T, &[u8]>::IS_NOT_ZST;
    unsafe {
        transmute_slice(val)
    }
}

///Gets byte slice out of the type.
///
///Available only for types that marked as `Pod`.
///
///## Usage
///
///```
///use lazy_bytes_cast::byte_slice_from;
///
///static BYTES: &[u8] = byte_slice_from(&0u32);
///```
///
///## Restrictions
///
///Compilation fails for invalid type.
///
///```compile_fail
///use lazy_bytes_cast::byte_slice_from;
///
///static BYTES: &[u8] = byte_slice_from(&());
///```
pub const fn byte_slice_from<T: Pod>(val: &T) -> &[u8] {
    let _ = Validator::<T, &[u8]>::IS_NOT_ZST;
    unsafe {
        transmute_slice(val)
    }
}

///Gets mutable byte slice out of the type.
///
///Available only for types that marked as `Pod`.
///
///## Usage
///
///```
///use lazy_bytes_cast::byte_slice_mut_from;
///
///let bytes = byte_slice_mut_from(&mut 0u32);
///```
///
///## Restrictions
///
///Compilation fails for invalid type.
///
///```compile_fail
///use lazy_bytes_cast::byte_slice_mut_from;
///
///let bytes = byte_slice_mut_from(&mut ());
///```
pub fn byte_slice_mut_from<T: Pod>(val: &mut T) -> &mut [u8] {
    let _ = Validator::<T, &[u8]>::IS_NOT_ZST;
    unsafe {
        transmute_slice_mut(val)
    }
}

///Reads `N` bytes from `Pod` object by performing `memcpy`
///
///## Usage
///
///```
///use lazy_bytes_cast::bytes_from;
///
///const INPUT: [u8; 4] = [123, 25, 99, 250];
///static BYTES: [u8; 4] = bytes_from(&u32::from_ne_bytes(INPUT));
///assert_eq!(BYTES, INPUT);
///
///static HALF: [u8; 2] = bytes_from(&u32::from_ne_bytes(INPUT));
///assert_eq!(HALF, &INPUT[..2]);
///```
///
///## Restrictions
///
///Compilation fails if `val` doesn't have enough bytes to read from.
///
///```compile_fail
///use lazy_bytes_cast::bytes_from;
///
///static BYTES: [u8; 5] = bytes_from(&0u32);
///```
pub const fn bytes_from<T: Pod, const N: usize>(val: &T) -> [u8; N] {
    let _ = Validator::<T, [u8; N]>::IS_ENOUGH_BYTES;
    unsafe {
        transmute_ref(val)
    }
}

///Reads `N` bytes from `Pod` object by performing `memcpy`
///
///## Usage
///
///```
///use lazy_bytes_cast::from_bytes;
///
///let input = 500_900_100u32;
///let res: u32 = from_bytes(&input.to_ne_bytes());
///assert_eq!(res, input);
///```
///
///## Restrictions
///
///Compilation fails if `val` is not the same size as output.
///
///```compile_fail,ignore
///use lazy_bytes_cast::from_bytes;
///
///let res: u32 = from_bytes(&[1, 2, 3]);
///```
pub fn from_bytes<T: Pod, const N: usize>(val: &[u8; N]) -> T {
    let _ = Validator::<[u8; N], T>::IS_SAME_SIZE;
    unsafe {
        Validator::<[u8; N], T>::READ_FN(val)
    }
}

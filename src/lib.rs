//!This crate provides simple methods to cast from and into byte arrays.
//!

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![no_std]

#[macro_export]
///Macro to read integer from bytes.
///
///## Params
///
///- `input` - name of byte array/slice, available in current scope;
///- `idx` - offset from where to read, assumed to be usize;
///- `as` - type of integer to return;
///
///## Usage
///
///```rust
///use lazy_bytes_cast::read_num;
///const BYTES: [u8; 5] = [255u8, 255u8, 255u8, 255u8, 0u8];
/////Const friendly
///const RESULT: u32 = read_num!(BYTES as u32);
///const OPTION_RESULT: Option<u32> = read_num!(try BYTES as u32);
///
///assert_eq!(RESULT, u32::max_value());
///assert_eq!(OPTION_RESULT, Some(RESULT));
///assert_eq!(read_num!(BYTES as u32), u32::max_value());
/////[idx] is offset from start
///assert_ne!(read_num!(BYTES[1] as u32), u32::max_value());
///
///assert!(read_num!(try BYTES as u32).is_some());
///assert!(read_num!(try BYTES[1] as u32).is_some());
///assert!(read_num!(try BYTES[2] as u32).is_none());
///assert!(read_num!(try BYTES[255] as u32).is_none());
///
///assert!(read_num!(try BYTES[2] as u8).is_some());
///assert!(read_num!(try BYTES[3] as u8).is_some());
///assert!(read_num!(try BYTES[4] as u8).is_some());
///assert!(read_num!(try BYTES[5] as u8).is_none());
///```
///
///## Panics
///
///On index out of bounds unless special `try` version is used, which returns `Option`
macro_rules! read_num {
    ($input:ident as $as:ty) => {
        $crate::read_num!($input[0] as $as)
    };
    (try $input:ident as $as:ty) => {
        $crate::read_num!(try $input[0] as $as)
    };
    ($input:ident[$idx:expr] as $as:ty) => {
        match $input {
            input => {
                let mut bytes = [0u8; core::mem::size_of::<$as>()];
                let mut idx = 0;
                let offset = $idx as usize;
                while idx < bytes.len() {
                    bytes[idx] = input[offset.wrapping_add(idx)];
                    idx = idx.wrapping_add(1);
                }

                <$as>::from_ne_bytes(bytes)
            }
        }
    };
    (try $input:ident[$idx:expr] as $as:ty) => {
        match $input {
            input => {
                const TYPE_LEN: usize = core::mem::size_of::<$as>();
                let offset = $idx as usize;
                if input.len().saturating_sub(offset) >= TYPE_LEN {
                    let mut bytes = [0u8; TYPE_LEN];
                    let mut idx = 0;
                    while idx < TYPE_LEN {
                        bytes[idx] = input[offset.wrapping_add(idx)];
                        idx = idx.wrapping_add(1);
                    }

                    Some(<$as>::from_ne_bytes(bytes))
                } else {
                    None
                }
            }
        }
    }
}

#[macro_export]
///Macro to manipulate bits within integer
///
///## Example
///
///```
///use lazy_bytes_cast::bit;
///
///let mut res = u32::max_value();
///assert!(!bit!(empty res));
///
///assert!(bit!(get res[1]));
///
///bit!(set res[1]);
///assert!(bit!(get res[1]));
///
///bit!(toggle res[1]);
///assert!(!bit!(get res[1]));
///
///bit!(unset res[1]);
///assert!(!bit!(get res[1]));
///
///bit!(toggle res[1]);
///assert!(bit!(get res[1]));
///bit!(reset res);
///assert_eq!(res, 0);
///
///assert!(!bit!(get res[1]));
///bit!(toggle res[1]);
///assert_ne!(res, 0);
///
///assert_eq!(bit!(size res), 4 * 8);
///let mut res = u32::min_value();
///
///assert!(bit!(empty res));
///assert!(!bit!(get res[1]));
///
///bit!(set res[1]);
///assert!(bit!(get res[1]));
///bit!(unset res[1]);
///assert!(!bit!(get res[1]));
///
///bit!(set res[26]);
///assert!(bit!(get res[26]));
///assert!(bit!(get res[90])); //90 % 32 == 26
///
///bit!(flip res);
///assert!(!bit!(get res[26]));
///assert!(bit!(get res[5]));
///```
macro_rules! bit {
    (get $input:ident[$idx:expr]) => {
        ($input.wrapping_shr($idx)) & 1 != 0
    };
    (set $input:ident[$idx:expr]) => {
        $input |= 1 << ($idx)
    };
    (unset $input:ident[$idx:expr]) => {
        $input &= !(1 << $idx)
    };
    (toggle $input:ident[$idx:expr]) => {
        $input ^= 1 << $idx
    };
    (empty $input:ident) => {
        $input == 0
    };
    (reset $input:ident) => {
        $input = 0
    };
    (flip $input:ident) => {
        $input = $input.reverse_bits()
    };
    (size $input:ident) => {
        core::mem::size_of_val(&$input) * 8
    }
    //($input:ident[$idx:expr]=$val:expr) => {
    //    $input = ($input & !(1 << $idx)) | (($val as _) << $idx);
    //}
}

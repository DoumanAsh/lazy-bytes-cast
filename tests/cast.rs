extern crate lazy_bytes_cast;

use lazy_bytes_cast::{
    ToBytesCast,
    FromBytesCast,
    FromBytesCastLazy,
    ByteSlice,
    ByteIndex
};

fn tester_bytes_to<T: ToBytesCast>(int_to_parse: T, expect: &[u8]) {
    let result = int_to_parse.to_bytes();

    assert!(result == expect);
}

#[test]
fn tests_bytes_to() {
    let run_data = vec![
        (50, [50, 0, 0, 0]),
        (9999999, [127, 150, 152, 0]),
        (u32::max_value(), [255, 255, 255, 255]),
        (0, [0, 0, 0, 0])
    ];

    for &(int_to_parse, expected) in run_data.iter() {
        tester_bytes_to(int_to_parse, &expected);
    }
}

fn tester_bytes_cast(bytes_to_parse: &[u8], expect: u32) {
    let result: Result<u32, ()> = bytes_to_parse.cast_to();

    assert!(result.is_ok());
    assert!(result.unwrap() == expect);
}

#[test]
fn test_bytes_from_common() {
    let run_data = vec![
        ([50, 0, 0, 0], 50),
        ([127, 150, 152, 0], 9999999),
        ([255, 255, 255, 255], u32::max_value()),
        ([0, 0, 0, 0], 0)
    ];

    for &(bytes_to_parse, expected) in run_data.iter() {
        tester_bytes_cast(&bytes_to_parse, expected);
    }
}

#[test]
fn test_bytes_from_fail() {
    let vec_data = vec![127, 150, 152, 0];
    let result: Result<u64, ()> = vec_data.cast_to();

    assert!(result.is_err());
}

#[test]
fn test_bytes_from_vec() {
    let vec_data = vec![127, 150, 152, 0];
    let expected: u32 = 9999999;

    let result: Result<u32, ()> = vec_data.cast_to();
    assert!(result.is_ok());
    assert!(result.unwrap() == expected);
}

#[test]
fn test_bytes_from_slice() {
    let slice = [127u8, 150, 152, 0];
    let expected: u32 = 9999999;

    let result: Result<u32, ()> = slice[..].cast_to();
    assert!(result.is_ok());
    assert!(result.unwrap() == expected);
}

#[test]
fn test_bytes_from_slice_ref() {
    let slice = [127u8, 150, 152, 0];
    let slice = &slice[..];
    let expected: u32 = 9999999;

    let result: Result<u32, ()> = slice.cast_to();
    assert!(result.is_ok());
    assert!(result.unwrap() == expected);
}

#[test]
fn test_bytes_from_array() {
    let arr: [u8; 4] = [127, 150, 152, 0];
    let expected: u32 = 9999999;

    let result: u32 = arr.cast_to();
    assert_eq!(result, expected);

    let arr = [1u8, 1];
    let expected: u16 = 257;

    let result: u16 = arr.cast_to();
    assert_eq!(result, expected);

    let arr = [1u8, 1, 1, 1, 1, 1, 2 , 1];
    let expected: u64 = 72621647814787329;

    let result: u64 = arr.cast_to();
    assert_eq!(result, expected);
}

#[test]
fn test_bytes_from_tuple() {
    let tuple = (127u8, 150, 152, 0);
    let expected: u32 = 9999999;

    let result: u32 = tuple.cast_to();
    assert_eq!(result, expected);

    let tuple = (1u8, 1);
    let expected: u16 = 257;

    let result: u16 = tuple.cast_to();
    assert_eq!(result, expected);

    let tuple = (1u8, 1, 1, 1, 1, 1, 2, 1);
    let expected: u64 = 72621647814787329;

    let result: u64 = tuple.cast_to();
    assert_eq!(result, expected);
}

#[test]
fn test_copy_to_bytes() {
    let mut arr = [0u8; 8];
    let var_from = 9999999;

    assert!(var_from.copy_to_bytes(&mut arr[1..]).is_ok());
    assert_eq!(arr, [0, 127, 150, 152, 0, 0, 0, 0])
}

#[test]
fn test_copy_to_bytes_err() {
    let mut arr = [0u8; 3];
    let var_from = 9999999;

    assert!(var_from.copy_to_bytes(&mut arr[1..]).is_err());
}

#[test]
fn test_byte_index() {
    let val_u32: u32 = 9999999;

    let result = val_u32.byte(0);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 127);

    let result = val_u32.byte(1);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 150);

    let result = val_u32.byte(2);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 152);

    let result = val_u32.byte(3);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 0);

    let result = val_u32.byte(4);
    assert!(result.is_none());
}

#[test]
fn test_byte_slice() {
    let expected = [127u8, 150, 152, 0];
    let var_from: u32 = 9999999;

    assert_eq!(var_from.byte_slice(), expected);
}

#[test]
fn test_byte_mut_slice() {
    let expected = [127u8, 150, 152, 0];
    let mut var_from: u32 = 9999999;

    assert_eq!(var_from.byte_mut_slice(), expected);

    var_from.byte_mut_slice()[0] = 1;
    assert_eq!(var_from, 9999873);

    var_from.byte_mut_slice()[1] = 1;
    assert_eq!(var_from, 9961729);

    var_from.byte_mut_slice()[2] = 1;
    assert_eq!(var_from, 65793);

    var_from.byte_mut_slice()[3] = 1;
    assert_eq!(var_from, 16843009);
}

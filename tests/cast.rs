extern crate lazy_bytes_cast;

use lazy_bytes_cast::{
    ToBytesCast,
    FromBytesCast
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
    let result: Result<u32, String> = bytes_to_parse.cast_to();

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
    let result: Result<u64, String> = vec_data.cast_to();

    assert!(result.is_err());
}

#[test]
fn test_bytes_from_vec() {
    let vec_data = vec![127, 150, 152, 0];
    let expected: u32 = 9999999;

    let result: Result<u32, String> = vec_data.cast_to();
    assert!(result.is_ok());
    assert!(result.unwrap() == expected);
}

#[test]
fn test_bytes_from_slice() {
    let slice = [127u8, 150, 152, 0];
    let expected: u32 = 9999999;

    let result: Result<u32, String> = slice[..].cast_to();
    assert!(result.is_ok());
    assert!(result.unwrap() == expected);
}

#[test]
fn test_bytes_from_slice_ref() {
    let slice = [127u8, 150, 152, 0];
    let slice = &slice[..];
    let expected: u32 = 9999999;

    let result: Result<u32, String> = slice.cast_to();
    assert!(result.is_ok());
    assert!(result.unwrap() == expected);
}

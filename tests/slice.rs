use lazy_bytes_cast::{FromByteArray, IntoByteArray, ByteSliceAsType, AsByteSlice};

#[test]
fn should_to_bytes() {
    let expected = [127u8, 150, 152, 0];
    let var_from: u32 = 9999999;

    assert_eq!(var_from.as_slice(), expected);
    assert_eq!(var_from.into_byte_array(), expected);
}

#[test]
fn should_from_bytes() {
    let expected: u32 = 9999999;
    let bytes = [127u8, 150, 152, 0];

    assert_eq!(*bytes.as_type::<u32>().unwrap(), expected);
    assert_eq!(u32::from_byte_array(bytes), expected);
}

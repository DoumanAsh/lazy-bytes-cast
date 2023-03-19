use lazy_bytes_cast::to_bytes;

#[test]
fn should_read_half_to_bytes() {
    let input = [123u8, 25, 99, 250];
    let half: [u8; 2] = to_bytes(&u32::from_ne_bytes(input));
    assert_eq!(half, &input[..2]);
}

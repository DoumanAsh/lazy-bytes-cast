use lazy_bytes_cast::bytes_from;

#[test]
fn should_read_half_bytes_from() {
    let input = [123u8, 25, 99, 250];
    let half: [u8; 2] = bytes_from(&u32::from_ne_bytes(input));
    assert_eq!(half, &input[..2]);
}

use lazy_bytes_cast::Read;

#[test]
fn should_read_bytes() {
    let input = [0u8, 255, 255, 255, 1];

    let mut read = Read::<u32>::new(&input);
    assert_eq!(read.size_hint(), (1, Some(1)));
    assert_eq!(read.next().unwrap(), u32::from_ne_bytes([0, 255, 255, 255]));
    assert_eq!(read.size_hint(), (0, Some(0)));
    assert!(read.next().is_none());
}

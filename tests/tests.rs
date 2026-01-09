use lazy_bytes_cast::{Read, Bits};
use lazy_bytes_cast::to_bytes;

#[test]
fn should_read_half_to_bytes() {
    let input = [123u8, 25, 99, 250];
    let half: [u8; 2] = to_bytes(&u32::from_ne_bytes(input));
    assert_eq!(half, &input[..2]);
}

#[test]
fn should_read_bytes() {
    let input = [0u8, 255, 255, 255, 1];

    let mut read = Read::<u32>::new(&input);
    assert_eq!(read.size_hint(), (1, Some(1)));
    assert_eq!(read.next().unwrap(), u32::from_ne_bytes([0, 255, 255, 255]));
    assert_eq!(read.size_hint(), (0, Some(0)));
    assert!(read.next().is_none());
}

#[test]
fn should_handle_bits() {
    let mut bits = Bits(0u16);
    assert!(!bits.get(0));
    assert_eq!(bits.0.count_ones(), 0);

    bits = bits.set(0);
    assert_eq!(bits.0.count_ones(), 1);

    assert!(bits.get(0));
    assert!(!bits.get(1));

    bits = bits.set(1);
    assert_eq!(bits.0.count_ones(), 2);
    assert!(bits.get(1));

    bits = bits.toggle(0);
    assert!(!bits.get(0));
    assert_eq!(bits.0.count_ones(), 1);

}

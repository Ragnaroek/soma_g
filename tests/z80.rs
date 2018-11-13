extern crate somag;

use somag::z80;

#[test]
fn test_u16_le() {
    let mem = [0x3C, 0x50, 0x01];
    let result = z80::u16_le(0, &mem);
    assert_eq!(result, 0x0150);
}

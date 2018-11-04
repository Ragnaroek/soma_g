
extern crate somag;

use somag::byte_string;

fn main() {
    // 3 tiles x 16 bytes
    let nintendo_logo = "CEED6666CC0D000B03730083000C000D0008111F8889000EDCCC6EE6DDDDD999BBBB67636E0EECCCDDDC999FBBB9333E";
    let bytes = byte_string::hex2_u8_array(nintendo_logo).unwrap();

    //0xce = 1100 1110
    //0xed = 1110 1101

    for i in 0..8 {
        let lo = bytes[2*i];
        let hi = bytes[2*i+1];
        let lo_v = (lo & (1 << i)) >> i;
        let hi_v = (hi & (1 << i)) >> i;
        let color_ix = lo_v + (hi_v << 1);
        //TODO Get RGB-Color for color_ix and write to pixel buffer (8x8)
    }
    //TODO convert pixel buffer to ppm
}


use std::io::{self, Error, ErrorKind};

pub fn hex2_u8_array(hex: &str) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    for i in 0..(hex.len() / 2) {
        let res = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16);
        match res {
            Ok(v) => bytes.push(v),
            Err(e) => {
                return Err(Error::new(ErrorKind::Other, format!("Problem with hex: {}", e)))
                //return Err()
                //error!("Problem with hex: {}", e);
                //return bytes;
            }
        };
    }
    Ok(bytes)
}

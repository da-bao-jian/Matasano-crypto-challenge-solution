use base64;
use std::u8;
use base64::{encode};

pub fn hex_to_base64(hex: String) -> String {
    // convert hex to a byte array
    let mut vec: Vec<u8> = Vec::new();
    for i in 0..hex.len()/2 {
        let byte = u8::from_str_radix(&hex[i*2..i*2+2], 16);
        match byte {
            Ok(r) => vec.push(r),
            Err(e) => println!("Error decoding: {}", e)
        }
    }

    encode(&vec)
}
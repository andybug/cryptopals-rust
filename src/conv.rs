use std::{io, str};

fn hex_to_bin(byte: u8) -> Result<u8, io::Error> {
    if byte >= 0x30 && byte <= 0x39 {
        return Ok(byte - 0x30);
    } else if byte >= 0x41 && byte <= 0x46 {
        return Ok(byte - 0x41 + 10);
    } else if byte >= 0x61 && byte <= 0x66 {
        return Ok(byte - 0x61 + 10);
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        format!("character '{:#02x}' is not valid", byte),
    ))
}

pub fn from_hex(hex: &str) -> Result<Vec<u8>, io::Error> {
    let hex_bytes = hex.as_bytes();

    if hex_bytes.len() % 2 == 1 {
        // odd number of hex digits is invalid
        // one byte of data = two hex digits
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "odd number of hex digits",
        ));
    }

    let mut bytes = Vec::<u8>::with_capacity(hex_bytes.len() / 2);
    for index in (0..hex_bytes.len()).step_by(2) {
        let high = hex_to_bin(hex_bytes[index])?;
        let low = hex_to_bin(hex_bytes[index + 1])?;
        let byte = ((high & 0xf) << 4) | (low & 0xf);
        bytes.push(byte);
    }

    Ok(bytes)
}

pub fn to_base64(bin: &[u8]) -> Result<String, io::Error> {
    const BASE64_DICT: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    //let mut b64str = String::with_capacity(4 * ((bin.len() + 2) / 3));
    let mut out_buf = Vec::<u8>::with_capacity(4 * ((bin.len() + 2) / 3));

    for i in (0..bin.len()).step_by(3) {
        let b0: u32 = if i < bin.len() { bin[i] as u32 } else { 0 };
        let b1: u32 = if i+1 < bin.len() { bin[i+1] as u32 } else { 0 };
        let b2: u32 = if i+2 < bin.len() { bin[i+2] as u32 } else { 0 };

        let combined = (b0 << 16) | (b1 << 8) | b2;

        out_buf.push(BASE64_DICT[((combined >> 18) & 0x3f) as usize] as u8);
        out_buf.push(BASE64_DICT[((combined >> 12) & 0x3f) as usize] as u8);
        out_buf.push(BASE64_DICT[((combined >> 6) & 0x3f) as usize] as u8);
        out_buf.push(BASE64_DICT[((combined >> 0) & 0x3f) as usize] as u8);

    }

    const PAD_TABLE: [usize; 3] = [0, 2, 1];
    for i in 0..PAD_TABLE[bin.len() % 3] {
        let out_index = out_buf.len() - 1 - i;
        out_buf[out_index] = '=' as u8;
    }

    Ok(String::from_utf8(out_buf).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex_length() {
        // test for invalid hex lengths
        assert_eq!(
            from_hex("123").unwrap_err().kind(),
            io::ErrorKind::InvalidInput
        );
    }

    #[test]
    fn from_hex_invalid() {
        assert_eq!(
            from_hex("12g3").unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn from_hex_bin_values() {
        assert_eq!(from_hex("67a4d3").unwrap().as_slice(), [0x67, 0xa4, 0xd3]);
    }

    #[test]
    fn to_base64_values() {
        assert_eq!(to_base64("hello".as_bytes()).unwrap(), "aGVsbG8=");
        assert_eq!(to_base64("M".as_bytes()).unwrap(), "TQ==");
        assert_eq!(to_base64("Ma".as_bytes()).unwrap(), "TWE=");
        assert_eq!(to_base64("Man".as_bytes()).unwrap(), "TWFu");
        assert_eq!(to_base64("Man ".as_bytes()).unwrap(), "TWFuIA==");
        assert_eq!(to_base64("Many hands make light work.".as_bytes()).unwrap(), "TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
    }
}
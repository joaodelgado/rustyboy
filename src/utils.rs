pub fn to_u32(b: &[u8]) -> u32 {
    assert!(b.len() >= 4);
    ((b[0] as u32) << 24) | ((b[1] as u32) << 16) | ((b[2] as u32) << 8) | (b[3] as u32)
}

pub fn to_u8_array(n: u32) -> [u8; 4] {
    let b1: u8 = ((n >> 24) & 0xff) as u8;
    let b2: u8 = ((n >> 16) & 0xff) as u8;
    let b3: u8 = ((n >> 8) & 0xff) as u8;
    let b4: u8 = (n & 0xff) as u8;
    return [b1, b2, b3, b4];
}

pub fn to_hex_string(bytes: &[u8]) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_u32() {
        assert_eq!(to_u32(&[0xff, 0xff, 0xff, 0xff]), 0xffffffff);
        assert_eq!(to_u32(&[0xff, 0x00, 0xff, 0x00]), 0xff00ff00);
    }

    #[test]
    fn test_to_u8_array() {
        assert_eq!(to_u8_array(0xffffffff), [0xff, 0xff, 0xff, 0xff]);
        assert_eq!(to_u8_array(0xff00ff00), [0xff, 0x00, 0xff, 0x00]);
    }
}

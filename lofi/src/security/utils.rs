use crate::LoFiError;

pub fn hex_to_bytes(s: &str) -> Result<Vec<u8>, LoFiError> {
    if s.len() % 2 != 0 {
        Err(LoFiError::InvalidLength)
    } else {
        let hey_bytes: Vec<&str> = (0..s.len()).step_by(2).map(|i| &s[i..i + 2]).collect();

        let mut bytes: Vec<u8> = Vec::new();

        for byte in hey_bytes {
            bytes.push(u8::from_str_radix(byte, 16).map_err(LoFiError::CouldNotParse)?)
        }
        Ok(bytes)
    }
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex: String = String::new();

    for byte in bytes {
        hex.push_str(&format!("{:x}", byte))
    }
    hex
}

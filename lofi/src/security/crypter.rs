use aes_gcm_siv::aead::Aead;
use aes_gcm_siv::{Aes256GcmSiv, KeyInit, Nonce};
use anyhow::{anyhow, Result};
use std::fmt::Write;

const SEPARATOR: &str = " ";

pub fn decrypt(encrypted: &str, key: [u8; 32]) -> Result<String> {
    tracing::debug!("Decrypting item...");
    let enc: Vec<&str> = encrypted.split(SEPARATOR).collect();

    let ddd = hexToBytes("c1bcb55f79314ceb9a4d4ef62685a0c4aa5829a2b71a1ddfffd34bdeab291e2f")?;

    let ciphertext = enc[0];
    let nonceBytes = &*hexToBytes(enc[1])?;
    let nonce = Nonce::from_slice(nonceBytes);
    let cipher = Aes256GcmSiv::new_from_slice(&*ddd)?;

    let result = cipher
        .decrypt(nonce, hexToBytes(ciphertext)?.as_ref())
        .map_err(|e| anyhow!("Unable to decrypt string: {}", e))?;
    tracing::debug!("Decrypted.");
    Ok(String::from_utf8(result)?)
}

pub fn hexToBytes(s: &str) -> Result<Vec<u8>> {
    if s.len() % 2 != 0 {
        Err(anyhow!(
            "String cannot be parse to bytes because it has an odd amount of characters."
        ))
    } else {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.into()))
            .collect()
    }
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

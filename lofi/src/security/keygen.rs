use crate::security::crypter::hexToBytes;
use anyhow::Result;
use scrypt::password_hash::rand_core::{OsRng, RngCore};
use scrypt::password_hash::{PasswordHasher, SaltString};
use scrypt::{scrypt, Scrypt};
use std::fmt::Write;

const SALT_LENGTH: usize = 32;
const VALIDATION_LENGTH: usize = 32;

pub fn derive_key(password: &str, salt: SaltString) -> Result<[u8; 32]> {
    let salt = salt.as_str();
    let salt_bytes = hexToBytes(salt)?;

    let mut key = [0u8; 32];

    tracing::debug!("Deriving key...");

    scrypt(
        password.as_bytes(),
        &*salt_bytes,
        &<Scrypt as PasswordHasher>::Params::new(18, 8, 1, 32)?,
        &mut key,
    )?;

    tracing::debug!("Key derived.");

    Ok(key)
}

pub fn generate_new_salt() -> SaltString {
    let mut salt = [0u8; SALT_LENGTH];
    OsRng.fill_bytes(&mut salt);
    SaltString::encode_b64(&salt).unwrap()
}

pub fn generate_new_validation() -> [u8; VALIDATION_LENGTH] {
    let mut validation = [0u8; VALIDATION_LENGTH];
    OsRng.fill_bytes(&mut validation);
    validation
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

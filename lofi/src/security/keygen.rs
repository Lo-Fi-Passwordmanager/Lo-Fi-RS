use crate::security::utils::{bytes_to_hex, hex_to_bytes};
use crate::LoFiError;
use aes_gcm_siv::aead::consts::U12;
use aes_gcm_siv::aead::generic_array::GenericArray;
use aes_gcm_siv::Nonce;
use anyhow::Result;
use scrypt::password_hash::rand_core::{OsRng, RngCore};
use scrypt::password_hash::PasswordHasher;
use scrypt::{scrypt, Scrypt};

const SALT_LENGTH: usize = 32;
const VALIDATION_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;

pub(super) fn derive_key(password: &str, salt: &str) -> Result<[u8; 32], LoFiError> {
    let salt_bytes = hex_to_bytes(salt)?;

    let mut key = [0u8; 32];

    tracing::debug!("Deriving key...");

    scrypt(
        password.as_bytes(),
        &salt_bytes,
        &<Scrypt as PasswordHasher>::Params::new(18, 8, 1, 32)
            .map_err(|_| LoFiError::InvalidScryptParams)?,
        &mut key,
    )
    .map_err(|_| LoFiError::InvalidScryptParams)?;

    tracing::debug!("Key derived.");

    Ok(key)
}

pub(super) fn generate_salt() -> String {
    let mut salt = [0u8; SALT_LENGTH];
    OsRng.fill_bytes(&mut salt);
    bytes_to_hex(&salt)
}

pub(super) fn generate_validation() -> String {
    let mut validation = [0u8; VALIDATION_LENGTH];
    OsRng.fill_bytes(&mut validation);
    bytes_to_hex(&validation)
}

pub(super) fn generate_nonce() -> GenericArray<u8, U12> {
    let mut nonce = [0u8; NONCE_LENGTH];
    OsRng.fill_bytes(&mut nonce);
    *Nonce::from_slice(&nonce)
}

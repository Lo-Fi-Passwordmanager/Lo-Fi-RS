use crate::document::automerge_document::AutomergeDoc;
use crate::security::keygen::{derive_key, generate_nonce};
use crate::security::utils::{bytes_to_hex, hex_to_bytes};
use crate::{LoFiError, LofiResult};
use aes_gcm_siv::aead::Aead;
use aes_gcm_siv::{Aes256GcmSiv, KeyInit, Nonce};

const SEPARATOR: &str = " ";
const NONCE_LENGTH: usize = 12;

pub struct Crypter {
    key: [u8; 32],
}

impl Crypter {
    pub fn from_doc(doc: &AutomergeDoc, password: &str) -> LofiResult<Self> {
        let key = derive_key(password, &doc.salt)?;
        let _ = Self::_decrypt(&key, &doc.validation).map_err(|_| LoFiError::InvalidPassword)?;
        Ok(Self { key })
    }

    pub fn decrypt(&self, encrypted: &str) -> LofiResult<String> {
        Self::_decrypt(&self.key, encrypted)
    }

    fn _decrypt(key: &[u8; 32], encrypted: &str) -> LofiResult<String> {
        tracing::debug!("Decrypting item...");
        let enc: Vec<&str> = encrypted.split(SEPARATOR).collect();

        if enc.len() != 2 {
            return Err(LoFiError::CouldNotDecrypt(
                "Value has no nonce.".to_string(),
            ));
        }

        let ciphertext = enc[0];
        let nonce_bytes = hex_to_bytes(enc[1])?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let cipher = Aes256GcmSiv::new_from_slice(key)
            .map_err(|_| LoFiError::CouldNotDecrypt("Derived key was to long.".to_string()))?;

        let decrypted = cipher
            .decrypt(nonce, hex_to_bytes(ciphertext)?.as_ref())
            .map_err(|_| LoFiError::CouldNotDecrypt("Unable to decrypt string.".to_string()))?;
        tracing::debug!("Decrypted.");
        Ok(String::from_utf8(decrypted).map_err(|_| {
            LoFiError::CouldNotDecrypt("Decrypted value is not a valid utf-8 string.".to_string())
        })?)
    }

    pub fn encrypt(&self, plaintext: &str) -> LofiResult<String> {
        let nonce = generate_nonce();
        let cipher = Aes256GcmSiv::new_from_slice(&self.key)
            .map_err(|_| LoFiError::CouldNotEncrypt("Derived key was to long.".to_string()))?;
        let encrypted = cipher
            .encrypt(&nonce, hex_to_bytes(plaintext)?.as_ref())
            .map_err(|_| LoFiError::CouldNotEncrypt("Unable to encrypt string.".to_string()))?;
        Ok(format!(
            "{}{}{}",
            bytes_to_hex(&encrypted),
            SEPARATOR,
            bytes_to_hex(&nonce)
        ))
    }
}

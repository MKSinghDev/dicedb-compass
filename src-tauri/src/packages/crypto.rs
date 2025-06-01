use base64::{engine::general_purpose, Engine};
use chacha20poly1305::{
    aead::{Aead, KeyInit, OsRng},
    AeadCore, ChaCha20Poly1305, Key, Nonce,
};

use super::error::AppError;

pub fn encrypt(data: &str, key_bytes: &[u8; 32]) -> Result<String, AppError> {
    let key = Key::from_slice(key_bytes);
    let cipher = ChaCha20Poly1305::new(key);

    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 12-byte random nonce
    let ciphertext = cipher.encrypt(&nonce, data.as_bytes())?;

    let nonce_b64 = general_purpose::STANDARD.encode(nonce.as_slice());
    let cipher_b64 = general_purpose::STANDARD.encode(ciphertext);
    Ok(format!("{nonce_b64}:{cipher_b64}"))
}

pub fn decrypt(enc: &str, key_bytes: &[u8; 32]) -> Result<String, AppError> {
    let key = Key::from_slice(key_bytes);
    let cipher = ChaCha20Poly1305::new(key);

    let parts: Vec<&str> = enc.split(':').collect();
    if parts.len() != 2 {
        return Err(AppError::InvalidFormat);
    }

    let nonce = general_purpose::STANDARD.decode(parts[0])?;
    let ciphertext = general_purpose::STANDARD.decode(parts[1])?;

    let nonce = Nonce::from_slice(&nonce);
    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref())?;
    Ok(String::from_utf8(plaintext)?)
}

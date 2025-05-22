use super::constants::{CHARSET, KEYRING_SERVICE, KEYRING_USERNAME};
use keyring::Entry;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

/// Generate a cryptographically secure random password
pub fn generate_secure_password(length: usize) -> String {
    let mut rng = ChaCha20Rng::from_rng(&mut rand::rng());

    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Store password in system keychain
pub async fn store_password_in_keychain(password: &str) -> Result<(), String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    entry
        .set_password(password)
        .map_err(|e| format!("Failed to store password in keychain: {}", e))?;

    Ok(())
}

/// Retrieve password from system keychain
pub async fn get_password_from_keychain() -> Result<String, String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    entry
        .get_password()
        .map_err(|e| format!("Failed to retrieve password from keychain: {}", e))
}

/// Delete password from system keychain
pub async fn delete_password_from_keychain() -> Result<(), String> {
    let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)
        .map_err(|e| format!("Failed to create keyring entry: {}", e))?;

    entry
        .delete_credential()
        .map_err(|e| format!("Failed to delete password from keychain: {}", e))?;

    Ok(())
}

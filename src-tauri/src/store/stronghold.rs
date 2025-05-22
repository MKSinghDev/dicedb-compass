use crate::utils::password::{
    delete_password_from_keychain, generate_secure_password, get_password_from_keychain,
    store_password_in_keychain,
};
use log::{error, info};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tauri_plugin_stronghold::stronghold::Stronghold;
use tokio::sync::Mutex;

/// Wrapper for thread-safe Stronghold operations
pub struct StrongholdState {
    pub stronghold: Arc<Mutex<Option<Stronghold>>>,
}

impl StrongholdState {
    pub fn new() -> Self {
        Self {
            stronghold: Arc::new(Mutex::new(None)),
        }
    }
}

/// Auto-initialize stronghold on app startup with keychain integration
pub async fn auto_init_stronghold(app_handle: AppHandle) -> Result<(), String> {
    let stronghold_state = app_handle.state::<StrongholdState>();
    let stronghold_path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("vault.stronghold");

    info!("Stronghold path {:?}", stronghold_path);
    // Check if stronghold file exists
    if stronghold_path.exists() {
        // Try to get password from keychain and unlock existing stronghold
        match get_password_from_keychain().await {
            Ok(password) => {
                info!("Found existing stronghold and password in keychain, unlocking...");
                let password_bytes = password.as_bytes().to_vec();

                match Stronghold::new(&stronghold_path, password_bytes) {
                    Ok(stronghold) => {
                        let mut state_guard = stronghold_state.stronghold.lock().await;
                        *state_guard = Some(stronghold);
                        info!("Successfully unlocked existing stronghold");
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to unlock stronghold with stored password: {}", e);
                        // Password might be wrong or corrupted, let user handle manually
                        Err(format!("Failed to unlock existing stronghold: {}", e))
                    }
                }
            }
            Err(_) => {
                error!("Existing stronghold found but no password in keychain");
                // Let user unlock manually
                Ok(())
            }
        }
    } else {
        // Create new stronghold with generated password
        info!("Creating new stronghold with auto-generated password...");

        // Generate secure random password (32 characters)
        let password = generate_secure_password(32);
        let password_bytes = password.as_bytes().to_vec();

        // Create new stronghold
        let stronghold = Stronghold::new(&stronghold_path, password_bytes)
            .map_err(|e| format!("Failed to create new stronghold: {}", e))?;

        // Store password in keychain
        store_password_in_keychain(&password).await?;

        // Store stronghold in state
        let mut state_guard = stronghold_state.stronghold.lock().await;
        *state_guard = Some(stronghold);

        info!("Successfully created new stronghold and stored password in keychain");
        Ok(())
    }
}

/// Command to manually initialize Stronghold with user-provided password
#[tauri::command]
pub async fn init_stronghold(
    app_handle: AppHandle,
    stronghold_state: tauri::State<'_, StrongholdState>,
    password: String,
    store_in_keychain: Option<bool>,
) -> Result<(), String> {
    let stronghold_path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("vault.stronghold");

    let password_bytes = password.as_bytes().to_vec();

    // Initialize the stronghold with password
    let stronghold = Stronghold::new(&stronghold_path, password_bytes)
        .map_err(|e| format!("Failed to create stronghold: {}", e))?;

    // Optionally store password in keychain for future auto-unlock
    if store_in_keychain.unwrap_or(false) {
        store_password_in_keychain(&password).await?;
    }

    // Store in the managed state
    let mut state_guard = stronghold_state.stronghold.lock().await;
    *state_guard = Some(stronghold);

    Ok(())
}

/// Command to unlock existing stronghold
#[tauri::command]
pub async fn unlock_stronghold(
    app_handle: AppHandle,
    stronghold_state: tauri::State<'_, StrongholdState>,
    password: String,
    store_in_keychain: Option<bool>,
) -> Result<(), String> {
    let stronghold_path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("vault.stronghold");

    if !stronghold_path.exists() {
        return Err("Stronghold file does not exist".to_string());
    }

    let password_bytes = password.as_bytes().to_vec();

    // Load existing stronghold with password
    let stronghold = Stronghold::new(&stronghold_path, password_bytes)
        .map_err(|e| format!("Failed to unlock stronghold: {}", e))?;

    // Optionally store password in keychain for future auto-unlock
    if store_in_keychain.unwrap_or(false) {
        store_password_in_keychain(&password).await?;
    }

    // Store in the managed state
    let mut state_guard = stronghold_state.stronghold.lock().await;
    *state_guard = Some(stronghold);

    Ok(())
}

/// Command to check if stronghold is ready
#[tauri::command]
pub async fn is_stronghold_ready(
    stronghold_state: tauri::State<'_, StrongholdState>,
) -> Result<bool, String> {
    let state_guard = stronghold_state.stronghold.lock().await;
    Ok(state_guard.is_some())
}

/// Command to reset stronghold (delete stronghold file and keychain entry)
#[tauri::command]
pub async fn reset_stronghold(
    app_handle: AppHandle,
    stronghold_state: tauri::State<'_, StrongholdState>,
) -> Result<(), String> {
    let stronghold_path = app_handle
        .path()
        .app_local_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?
        .join("vault.stronghold");

    // Clear from memory
    {
        let mut state_guard = stronghold_state.stronghold.lock().await;
        *state_guard = None;
    }

    // Delete stronghold file if it exists
    if stronghold_path.exists() {
        std::fs::remove_file(&stronghold_path)
            .map_err(|e| format!("Failed to delete stronghold file: {}", e))?;
    }

    // Delete password from keychain (ignore errors as it might not exist)
    let _ = delete_password_from_keychain().await;

    Ok(())
}

/// Command to check if password exists in keychain
#[tauri::command]
pub async fn has_stored_password() -> Result<bool, String> {
    match get_password_from_keychain().await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Command to save a secret in the Stronghold vault
#[tauri::command]
pub async fn save_secret(
    stronghold_state: tauri::State<'_, StrongholdState>,
    key: String,
    value: String,
) -> Result<(), String> {
    let state_guard = stronghold_state.stronghold.lock().await;

    if let Some(stronghold) = state_guard.as_ref() {
        stronghold
            .store()
            .insert(key.as_bytes().to_vec(), value.as_bytes().to_vec(), None)
            .map_err(|e| format!("Failed to save secret: {}", e))?;

        stronghold
            .save()
            .map_err(|e| format!("Failed to save stronghold: {}", e))?;

        Ok(())
    } else {
        Err("Stronghold not initialized. Call init_stronghold first.".to_string())
    }
}

/// Command to retrieve a secret from the Stronghold vault
#[tauri::command]
pub async fn get_secret(
    stronghold_state: tauri::State<'_, StrongholdState>,
    key: String,
) -> Result<String, String> {
    let state_guard = stronghold_state.stronghold.lock().await;

    if let Some(stronghold) = state_guard.as_ref() {
        let data = stronghold
            .store()
            .get(key.as_bytes())
            .map_err(|e| format!("Failed to get secret: {}", e))?;

        let data = match data {
            Some(data) => data,
            None => return Err(format!("Secret not found: {}", key)),
        };

        String::from_utf8(data).map_err(|e| format!("Failed to convert secret to string: {}", e))
    } else {
        Err("Stronghold not initialized. Call init_stronghold first.".to_string())
    }
}

#[tauri::command]
pub async fn get_keys(
    stronghold_state: tauri::State<'_, StrongholdState>,
) -> Result<Vec<String>, String> {
    let state_guard = stronghold_state.stronghold.lock().await;

    if let Some(stronghold) = state_guard.as_ref() {
        let keys = stronghold
            .store()
            .keys()
            .map_err(|e| format!("Failed to get keys: {}", e))?;

        let keys = keys
            .iter()
            .map(|key| {
                let key = String::from_utf8(key.to_vec())
                    .map_err(|e| format!("Failed to convert key to string: {}", e))?;
                Ok(key)
            })
            .collect::<Result<Vec<String>, String>>()?;

        Ok(keys)
    } else {
        Err("Stronghold not initialized. Call init_stronghold first.".to_string())
    }
}

/// Command to remove a secret from the Stronghold vault
#[tauri::command]
pub async fn remove_secret(
    stronghold_state: tauri::State<'_, StrongholdState>,
    key: String,
) -> Result<(), String> {
    let state_guard = stronghold_state.stronghold.lock().await;

    if let Some(stronghold) = state_guard.as_ref() {
        stronghold
            .store()
            .delete(key.as_bytes())
            .map_err(|e| format!("Failed to remove secret: {}", e))?;

        stronghold
            .save()
            .map_err(|e| format!("Failed to save stronghold: {}", e))?;

        Ok(())
    } else {
        Err("Stronghold not initialized. Call init_stronghold first.".to_string())
    }
}

/// Command to check if stronghold is initialized
#[tauri::command]
pub async fn is_stronghold_initialized(
    stronghold_state: tauri::State<'_, StrongholdState>,
) -> Result<bool, String> {
    let state_guard = stronghold_state.stronghold.lock().await;
    Ok(state_guard.is_some())
}

/// Command to lock/clear stronghold from memory
#[tauri::command]
pub async fn lock_stronghold(
    stronghold_state: tauri::State<'_, StrongholdState>,
) -> Result<(), String> {
    let mut state_guard = stronghold_state.stronghold.lock().await;
    *state_guard = None;
    Ok(())
}

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::Manager;

use crate::commands::auth::response_struct::LoginResponse;
use crate::keys::PrivateKeys;
use crate::{log_error, log_info, log_warn};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[tauri::command]
pub async fn login(
    payload: LoginPayload,
    app_handle: tauri::AppHandle,
) -> Result<LoginResponse, String> {
    let client = Client::new();

    let res = client
        .post("http://localhost:8000/auth/login")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Erreur lors de la requête: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Échec: HTTP {}", res.status()));
    }

    let data: LoginResponse = res
        .json()
        .await
        .map_err(|e| format!("Erreur parsing JSON: {}", e))?;

    let _keys = load_private_keys(&app_handle, &payload.username)?;
    log_info!(
        "Private keys loaded successfully after login for user {}",
        payload.username
    );
    Ok(data)
}

fn load_private_keys(app_handle: &tauri::AppHandle, username: &str) -> Result<PrivateKeys, String> {
    let dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("lucchat");
    let file_path = dir.join(format!("{}_keys.json", username));

    if !file_path.exists() {
        log_warn!("Private keys file not found");
        return Err("❌ Private keys not found. Please register first.".to_string());
    }

    let data =
        fs::read_to_string(&file_path).map_err(|e| format!("Failed to read keys file: {}", e))?;
    let keys: PrivateKeys =
        serde_json::from_str(&data).map_err(|e| format!("Failed to parse keys file: {}", e))?;
    log_error!("Failed to load private keys");
    Ok(keys)
}

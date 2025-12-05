use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::commands::auth::response_struct::LoginResponse;
use crate::log_info;
use crate::crypt::x3dh::session::Session;
use crate::commands::message::send_message::{send_message, SendMessageRequest};

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

    let _keys = data.user.load_private_keys(&app_handle)?;
    log_info!(
        "Private keys loaded successfully after login for user {}",
        payload.username
    );
    let _sessions = Session::load_private_sessions(&app_handle, &payload.username)?;
    log_info!(
        "Private sessions loaded successfully after login for user {}",
        payload.username
    );
    Ok(data)
}

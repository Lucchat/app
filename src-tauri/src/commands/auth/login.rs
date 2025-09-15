use serde::{Deserialize, Serialize};
use reqwest::Client;

use crate::commands::auth::response_struct::LoginResponse;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[tauri::command]
pub async fn login(payload: LoginPayload) -> Result<LoginResponse, String> {
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

    Ok(data)
}

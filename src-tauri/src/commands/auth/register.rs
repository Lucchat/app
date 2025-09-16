use crate::commands::auth::response_struct::LoginResponse;
use crate::keys::one_time_prekey::OneTimePreKeyPublic;
use crate::keys::PrivateKeys;
use crate::keys::{
    identity::IdentityKey, one_time_prekey::OneTimePreKeyGroup, signed_prekey::SignedPreKey,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::Manager;

#[derive(Deserialize, Serialize)]
pub struct RegisterPayloadFromFrontend {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct RegisterPayload {
    username: String,
    password: String,
    ik_pub: [u8; 32],
    spk_pub: [u8; 32],
    opk_pub: Vec<OneTimePreKeyPublic>,
}

#[tauri::command]
pub async fn register(
    payload: RegisterPayloadFromFrontend,
    app_handle: tauri::AppHandle,
) -> Result<LoginResponse, String> {
    let ik = IdentityKey::new();
    let spk = SignedPreKey::new(&ik.signing_key());
    let opk = OneTimePreKeyGroup::new(100);

    let payload_with_key = RegisterPayload {
        username: payload.username.clone(),
        password: payload.password.clone(),
        ik_pub: ik.dh_public,
        spk_pub: spk.public,
        opk_pub: opk.public_group().keys,
    };

    let client = Client::new();

    let res = client
        .post("http://localhost:8000/auth/register")
        .json(&payload_with_key)
        .send()
        .await
        .map_err(|e| format!("Erreur lors de la requête: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Échec: HTTP {}", res.status()));
    }
    save_private_keys(
        &app_handle,
        &PrivateKeys { ik, spk, opk },
        &payload.username,
    )?;
    let data: LoginResponse = res
        .json()
        .await
        .map_err(|e| format!("Erreur parsing JSON: {}", e))?;

    Ok(data)
}

fn save_private_keys(
    app_handle: &tauri::AppHandle,
    keys: &PrivateKeys,
    username: &str,
) -> Result<(), String> {
    let dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir")
        .join("lucchat");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let file_path = dir.join(format!("{}_keys.json", username));

    println!("🔑 Saving keys to: {:?}", file_path);

    let data = serde_json::to_string(keys).map_err(|e| e.to_string())?;
    std::fs::write(&file_path, data).map_err(|e| e.to_string())
}

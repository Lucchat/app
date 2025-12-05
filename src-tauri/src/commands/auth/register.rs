use crate::commands::auth::response_struct::LoginResponse;
use crate::crypt::x3dh::session::Session;
use crate::structs::keys::one_time_prekey::OneTimePreKeyPublic;
use crate::structs::keys::PrivateKeys;
use crate::structs::keys::{
    identity::IdentityKey, one_time_prekey::OneTimePreKeyGroup, signed_prekey::SignedPreKey,
};
use crate::log_info;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterPayload {
    username: String,
    password: String,
    ik_pub: [u8; 32],
    spk_pub: [u8; 32],
    opks_pub: Vec<OneTimePreKeyPublic>,
}

#[tauri::command]
pub async fn register(
    payload: RegisterRequest,
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
        opks_pub: opk.public_group().keys,
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
    let data: LoginResponse = res
        .json()
        .await
        .map_err(|e| format!("Erreur parsing JSON: {}", e))?;
    log_info!("User {} registered successfully", payload.username);
    data.user.create_private_keys(
        &app_handle,
        &PrivateKeys { ik, spk, opk },
    )?;
    Session::create_file_sessions(&app_handle, &payload.username);
    log_info!(
        "Private keys and sessions file created successfully for user {}",
        payload.username
    );
    Ok(data)
}

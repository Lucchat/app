use reqwest::Client;
use tauri::Manager;
use std::fs::{self, OpenOptions};
use std::io::Write;
use serde::{Deserialize, Serialize};
use crate::commands::auth::response_struct::LoginResponse;
use crate::keys::one_time_prekey::OneTimePreKeyPublic;
use crate::keys::{identity::IdentityKey, one_time_prekey::OneTimePreKeyGroup, signed_prekey::SignedPreKey};

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
pub async fn register(payload: RegisterPayloadFromFrontend, app_handle: tauri::AppHandle) -> Result<LoginResponse, String> {
    // 1. Vérifier si le fichier local existe
    // let dir = app_handle.path().app_data_dir().expect("Failed to get app data dir").join("lucchat");
    // let file_path = dir.join("users.txt");

    // // 2. Créer le dossier si besoin
    // if let Some(parent) = file_path.parent() {
    //     fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    // }
    let ik = IdentityKey::new();
    let spk = SignedPreKey::new(&ik.signing_key());
    let opk = OneTimePreKeyGroup::new(100); // Générer

    let payload_with_key = RegisterPayload {
        username: payload.username.clone(),
        password: payload.password.clone(),
        ik_pub: ik.dh_public,
        spk_pub: spk.public,
        opk_pub: opk.public_group().keys,
    };
    
    println!("ik: {:?}", ik.dh_public);
    println!("spk: {:?}", spk.public);
    println!("opks: {:?}", opk.public_group().keys);

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

    Ok(data)
}

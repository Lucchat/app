use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: Tokens,
    // ajoute d’autres champs selon l’API (refresh_token, etc.)
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Tokens {
    pub access: String,
    pub refresh: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub description: Option<String>,
    pub profile_picture: Option<String>,
    pub pending_friend_requests: Vec<String>,
    pub friends_requests: Vec<String>,
    pub friends: Vec<String>,
    pub keys: Key,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    pub ik_pub: [u8; 32],
    pub spk_pub: [u8; 32],
    pub opk_pub: Vec<[u8; 32]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub uuid: String,
    pub sender: String,
    pub receiver: String,
    pub nonce: [u8; 12],
    pub ciphertext: Vec<u8>,
    pub ratchet_pub: [u8; 32], // DH public key used in ratchet step
    pub message_index: u32,    // Index in chain key (CKs.index)
    pub opk_used: Option<[u8; 32]>,
    pub ek_used: Option<[u8; 32]>,
    pub created_at: i64,
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

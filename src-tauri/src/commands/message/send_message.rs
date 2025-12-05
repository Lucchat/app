use crate::crypt::{
    crypto_utils::hkdf::derive_root_key, double_ratchet::state::RatchetState,
    x3dh::session::Session,
};
use crate::structs::keys::one_time_prekey::OneTimePreKeyPublic;
use crate::structs::keys::PrivateKeys;
use crate::structs::keys::{
    ephemeral_key::EphemeralKey, one_time_prekey::OneTimePreKeyGroup, ratchet_key::RatchetKey,
    signed_prekey::SignedPreKey,
};
use crate::structs::message::Message;
use crate::structs::user::{User, UserPublicFriend};
use crate::log_info;

use serde::{Deserialize, Serialize};

use reqwest::Client;

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub jwt_access: String,
    pub uuid_to: String,
    pub plaintext: String,
}

#[derive(Serialize, Debug)]
struct SendMessagePayload {
    pub uuid: String,
    pub sender: String,
    pub receiver: String,
    pub nonce: [u8; 12],
    pub ciphertext: Vec<u8>,
    pub ratchet_pub: [u8; 32],
    pub message_index: u32,
    pub opk_used: Option<OneTimePreKeyPublic>,
    pub ek_used: Option<[u8; 32]>,
    pub created_at: i64,
}


/// Sends a message to the target user using their [`UserPublicFriend`].
///
/// If no session exists, initializes a new one using the X3DH protocol, followed by
/// Double Ratchet encryption of the plaintext.
///
/// # Arguments
/// - `to`: Public info of the recipient user.
/// - `plaintext`: Message content to encrypt.
///
/// # Returns
/// An [`Message`] ready for transmission.
#[tauri::command]
pub async fn send_message(
    payload: SendMessageRequest,
    app_handle: &tauri::AppHandle,
) -> Result<(), String> {
    let me = get_my_user(payload.jwt_access.clone()).await.unwrap();
    let to = get_user_public_friend(payload.jwt_access.clone(), payload.uuid_to.clone())
        .await
        .unwrap();
    let receiver_id = payload.uuid_to.clone();
    let mut used_opk: Option<OneTimePreKeyPublic> = None;
    let mut used_ek: Option<[u8; 32]> = None;
    let mut sessions = Session::load_private_sessions(&app_handle, &me.username)
        .expect("Failed to load private sessions");
    let ratchet = sessions.entry(receiver_id.clone()).or_insert_with(|| {
        let ek = EphemeralKey::new();
        let opk = to.keys.opks_pub.get(0); // In real implementation, select an unused one-time pre-key
        let me_private_keys: PrivateKeys = me
            .load_private_keys(&app_handle.clone())
            .expect("Failed to load private keys");
        used_ek = Some(ek.public);
        let session = Session::create_session_key(
            me.uuid.clone(),
            to.uuid.clone(),
            &me_private_keys.ik,
            &ek,
            to.keys.spk_pub,
            to.keys.ik_pub,
            opk,
        );
        let rk = derive_root_key(&session.get_bytes());
        let dhs = RatchetKey::new();
        used_opk = opk.cloned();
        used_ek = Some(ek.public);
        RatchetState::new(rk, dhs, Some(to.keys.spk_pub), true)
    });
    let message = ratchet.encrypt(
        &payload.plaintext,
        me.uuid.clone(),
        to.uuid.clone(),
        used_opk,
        used_ek,
    );
    Session::save_sessions(app_handle, &sessions, &me.username)?;
    let payload_with_key = SendMessagePayload {
        uuid: message.uuid,
        sender: message.sender,
        receiver: message.receiver,
        nonce: message.nonce,
        ciphertext: message.ciphertext,
        ratchet_pub: message.ratchet_pub,
        message_index: message.message_index,
        opk_used: message.opk_used,
        ek_used: message.ek_used,
        created_at: message.created_at,
    };

    let client = Client::new();
    let res = client
        .post("http://localhost:8000/message/send")
        .bearer_auth(payload.jwt_access.clone())
        .json(&payload_with_key)
        .send()
        .await
        .map_err(|e| format!("Erreur lors de la requête: {}", e))?;

    if !res.status().is_success() {
        let status = res.status();
        let err_text = res.text().await.unwrap_or("<no body>".into());
        return Err(format!("Échec: HTTP {}", status));
    }

    log_info!("Message sent successfully with status: {}", res.status());
    Ok(())
    // ! envoyer a la bdd via l'api et enregistrer la nouvelle session créer //
}

async fn get_my_user(token: String) -> Result<User, String> {
    let client = Client::new();

    let res = client
        .get("http://localhost:8000/user/me")
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Erreur lors de la requête: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Échec: HTTP {}", res.status()));
    }
    let data: User = res
        .json()
        .await
        .map_err(|e| format!("Erreur parsing JSON: {}", e))?;
    Ok(data)
}

async fn get_user_public_friend(token: String, uuid: String) -> Result<UserPublicFriend, String> {
    let client = Client::new();

    let res = client
        .get(&format!("http://localhost:8000/user/{}", uuid))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| format!("Erreur lors de la requête: {}", e))?;

    if !res.status().is_success() {
        return Err(format!("Échec: HTTP {}", res.status()));
    }
    let data: UserPublicFriend = res
        .json()
        .await
        .map_err(|e| format!("Erreur parsing JSON: {}", e))?;
    Ok(data)
}

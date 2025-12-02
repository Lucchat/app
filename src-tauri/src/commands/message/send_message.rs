use crate::structs::keys::{
    one_time_prekey::OneTimePreKeyGroup, ratchet_key::RatchetKey, signed_prekey::SignedPreKey,
    ephemeral_key::EphemeralKey,
};
use crate::structs::user::{User, UserPublicFriend};
use crate::structs::message::Message;
use crate::structs::keys::PrivateKeys;
use crate::crypt::{
    crypto_utils::hkdf::derive_root_key,
    double_ratchet::state::RatchetState,
    x3dh::session::Session,
};

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
pub fn send_message(me: &mut User, to: &UserPublicFriend, plaintext: &str, app_handle: &tauri::AppHandle,) -> Message {
    let receiver_id = to.uuid.clone();
    let mut used_opk: Option<[u8; 32]> = None;
    let mut used_ek: Option<[u8; 32]> = None;
    let mut sessions = Session::load_private_sessions(&app_handle, &me.username).expect("Failed to load private sessions");
    let ratchet = sessions.entry(receiver_id.clone()).or_insert_with(|| {
        let ek = EphemeralKey::new();
        let opk = to.keys.opks_pub.get(0); // In real implementation, select an unused one-time pre-key
        let me_private_keys: PrivateKeys = me.load_private_keys(&app_handle.clone()).expect("Failed to load private keys");
        used_ek = Some(ek.public);
        let session = Session::create_session_key(
            me.username.clone(),
            to.username.clone(),
            &me_private_keys.ik,
            &ek,
            to.keys.spk_pub,
            to.keys.ik_pub,
            opk,
        );
        let rk = derive_root_key(&session.get_bytes());
        let dhs = RatchetKey::new();
        RatchetState::new(rk, dhs, Some(to.keys.spk_pub), true)
    });
    ratchet.encrypt(
        plaintext,
        me.username.clone(),
        to.username.clone(),
        used_opk,
        used_ek,
    )
    // ! envoyer a la bdd via l'api et enregistrer la nouvelle session créer //
}
use crate::crypt::crypto_utils::{dh::diffie_hellman, hkdf::derive_session_key};
use crate::crypt::double_ratchet::state::RatchetState;
use crate::structs::keys::one_time_prekey::OneTimePreKey;
use crate::structs::keys::{
    ephemeral_key::EphemeralKey, identity::IdentityKey, one_time_prekey::OneTimePreKeyPublic,
    session_key::SessionKey, signed_prekey::SignedPreKey,
};
use crate::{log_error, log_info, log_warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session;

impl Session {
    pub fn load_private_sessions(
        app_handle: &tauri::AppHandle,
        username: &str,
    ) -> Result<HashMap<String, RatchetState>, String> {
        let dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir")
            .join("lucchat");
        let file_path = dir.join(format!("{}_sessions.json", username));

        if !file_path.exists() {
            log_warn!("Private sessions file not found");
            return Err("❌ Private sessions not found. Please register first.".to_string());
        }

        let data = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read sessions file: {}", e))?;

        let sessions: HashMap<String, RatchetState> = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse sessions file: {}", e))?;
        log_info!("Private sessions loaded successfully");
        Ok(sessions)
    }

    pub fn create_file_sessions(
        app_handle: &tauri::AppHandle,
        username: &str,
    ) -> HashMap<String, RatchetState> {
        let dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir")
            .join("lucchat");
        if let Err(e) = std::fs::create_dir_all(&dir) {
            log_error!("Failed to create directory: {}", e);
            return HashMap::new();
        }
        let file_path = dir.join(format!("{}_sessions.json", username));
        let empty_sessions: HashMap<String, RatchetState> = HashMap::new();
        let data = match serde_json::to_string(&empty_sessions) {
            Ok(d) => d,
            Err(e) => {
                log_error!("Failed to serialize empty sessions: {}", e);
                return HashMap::new();
            }
        };
        if let Err(e) = std::fs::write(&file_path, data) {
            log_error!("Failed to write sessions file: {}", e);
            return HashMap::new();
        }
        log_info!(
            "Empty private sessions file created successfully for user {}",
            username
        );
        empty_sessions
    }

    pub fn save_sessions(
        app_handle: &tauri::AppHandle,
        sessions: &HashMap<String, RatchetState>,
        username: &str,
    ) -> Result<(), String> {
        let dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir")
            .join("lucchat");
        if let Err(e) = std::fs::create_dir_all(&dir) {
            log_error!("Failed to create directory: {}", e);
            return Err(e.to_string());
        }
        let file_path = dir.join(format!("{}_sessions.json", username));
        let data = match serde_json::to_string(sessions) {
            Ok(d) => d,
            Err(e) => {
                log_error!("Failed to serialize sessions: {}", e);
                return Err(e.to_string());
            }
        };
        if let Err(e) = std::fs::write(&file_path, data) {
            log_error!("Failed to write sessions file: {}", e);
            return Err(e.to_string());
        }
        log_info!("Private sessions saved successfully for user {}", username);
        Ok(())
    }

    /// Creates a new session key for the initiator (sender) in the X3DH protocol.
    ///
    /// Performs the required Diffie-Hellman (DH) operations between the initiator's identity
    /// and ephemeral keys and the receiver's signed prekey, identity key, and optionally,
    /// one-time prekey. These shared secrets are combined and passed through HKDF to derive the session key.
    ///
    /// # Parameters:
    /// - `sender_name`: Name or ID of the sender (initiator).
    /// - `receiver_name`: Name or ID of the receiver.
    /// - `ik_initiator`: Initiator's identity key.
    /// - `ek_initiator`: Initiator's ephemeral key.
    /// - `spk_receiver`: Receiver's signed pre-key (public).
    /// - `ik_receiver`: Receiver's identity key (public).
    /// - `opk_receiver`: Optional one-time pre-key (public).
    ///
    /// # Returns
    /// A [`SessionKey`] object containing the derived shared secret and participant metadata.
    ///
    /// # X3DH DH Computations:
    /// - DH1: IK_initiator <-> SPK_receiver
    /// - DH2: EK_initiator <-> IK_receiver
    /// - DH3: EK_initiator <-> SPK_receiver
    /// - DH4: EK_initiator <-> OPK_receiver (if present)
    ///
    /// # Panics
    /// May panic if any cryptographic primitive fails unexpectedly.
    pub fn create_session_key(
        sender_name: String,
        receiver_name: String,
        ik_initiator: &IdentityKey,
        ek_initiator: &EphemeralKey,
        spk_receiver: [u8; 32],
        ik_receiver: [u8; 32],
        opk_receiver: Option<&OneTimePreKeyPublic>,
    ) -> SessionKey {
        let dh1 = diffie_hellman(&ik_initiator.get_private(), &spk_receiver);
        let dh2 = diffie_hellman(&ek_initiator.get_private(), &ik_receiver);
        let dh3 = diffie_hellman(&ek_initiator.get_private(), &spk_receiver);

        let mut ikm = Vec::new();
        ikm.extend_from_slice(&dh1);
        ikm.extend_from_slice(&dh2);
        ikm.extend_from_slice(&dh3);

        if let Some(opk) = opk_receiver {
            let dh4 = diffie_hellman(&ek_initiator.get_private(), &opk.key);
            ikm.extend_from_slice(&dh4);
        }

        let sk_bytes = derive_session_key(&ikm);

        SessionKey::new(sk_bytes, sender_name, receiver_name)
    }

    /// Derives a session key for the receiver (responder) in the X3DH protocol.
    ///
    /// Performs the required Diffie-Hellman (DH) operations using the receiver’s identity,
    /// signed pre-key, and one-time pre-key, in combination with the sender’s identity
    /// and ephemeral public keys. The output is a shared `SessionKey` derived via HKDF.
    ///
    /// # Parameters:
    /// - `receiver_name`: Receiver's ID or name.
    /// - `sender_name`: Sender's ID or name.
    /// - `receiver_ik`: Receiver’s identity key (private).
    /// - `receiver_spk`: Receiver’s signed pre-key (private).
    /// - `receiver_opk`: Receiver’s one-time pre-key (private).
    /// - `sender_ik_public`: Sender's identity key (public).
    /// - `sender_ek_public`: Sender's ephemeral key (public).
    ///
    /// # Returns
    /// A [`SessionKey`] derived from the DH shared secrets.
    ///
    /// # X3DH DH Computations:
    /// - DH1: SPK_receiver <-> IK_sender
    /// - DH2: IK_receiver <-> EK_sender
    /// - DH3: SPK_receiver <-> EK_sender
    /// - DH4: OPK_receiver <-> EK_sender
    ///
    /// # Panics
    /// May panic if any of the internal cryptographic functions fail unexpectedly.
    pub fn receive_session_key(
        receiver_name: String,
        sender_name: String,
        receiver_ik: &IdentityKey,
        receiver_spk: &SignedPreKey,
        receiver_opk: OneTimePreKey,
        sender_ik_public: [u8; 32],
        sender_ek_public: [u8; 32],
    ) -> SessionKey {
        let dh1 = diffie_hellman(&receiver_spk.get_private(), &sender_ik_public);
        let dh2 = diffie_hellman(&receiver_ik.get_private(), &sender_ek_public);
        let dh3 = diffie_hellman(&receiver_spk.get_private(), &sender_ek_public);
        let dh4 = diffie_hellman(&receiver_opk.get_private(), &sender_ek_public);

        let ikm = [dh1, dh2, dh3, dh4].concat();
        let sk_bytes = derive_session_key(&ikm);

        SessionKey::new(sk_bytes, sender_name, receiver_name)
    }
}

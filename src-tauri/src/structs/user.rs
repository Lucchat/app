use crate::structs::keys::one_time_prekey::OneTimePreKeyPublic;
use crate::structs::keys::PrivateKeys;
use crate::crypt::x3dh::session::Session;
use crate::{log_error, log_info, log_warn};

use serde::{Deserialize, Serialize};
use std::fs;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    pub ik_pub: [u8; 32],
    pub spk_pub: [u8; 32],
    pub opks_pub: Vec<OneTimePreKeyPublic>,
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

impl User {
    pub fn load_private_keys(&self, app_handle: &tauri::AppHandle) -> Result<PrivateKeys, String> {
        let dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir")
            .join("lucchat");
        let file_path = dir.join(format!("{}_keys.json", self.username));

        if !file_path.exists() {
            log_warn!("Private keys file not found");
            return Err("❌ Private keys not found. Please register first.".to_string());
        }

        let data = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read keys file: {}", e))?;
        let keys: PrivateKeys =
            serde_json::from_str(&data).map_err(|e| format!("Failed to parse keys file: {}", e))?;
        Ok(keys)
    }

    pub fn create_private_keys(
        &self,
        app_handle: &tauri::AppHandle,
        keys: &PrivateKeys,
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
        let file_path = dir.join(format!("{}_keys.json", self.username));
        let data = match serde_json::to_string(keys) {
            Ok(d) => d,
            Err(e) => {
                log_error!("Failed to serialize keys: {}", e);
                return Err(e.to_string());
            }
        };
        if let Err(e) = std::fs::write(&file_path, data) {
            log_error!("Failed to write keys file: {}", e);
            return Err(e.to_string());
        }
        log_info!("Private keys saved successfully for user {}", self.username);
        Ok(())
    }

    pub fn load_private_sessions(
        &self,
        app_handle: &tauri::AppHandle,
    ) -> Result<Session, String> {
        let dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app data dir")
            .join("lucchat");
        let file_path = dir.join(format!("{}_sessions.json", self.username));

        if !file_path.exists() {
            log_warn!("Private sessions file not found");
            return Err("❌ Private sessions not found. Please register first.".to_string());
        }

        let data = fs::read_to_string(&file_path)
            .map_err(|e| format!("Failed to read sessions file: {}", e))?;

        let sessions: Session = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse sessions file: {}", e))?;
        log_info!("Private sessions loaded successfully");
        Ok(sessions)
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UserResponse {
    Public(UserPublic),
    PublicFriend(UserPublicFriend),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPublic {
    pub uuid: String,
    pub username: String,
    pub description: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPublicFriend {
    pub uuid: String,
    pub username: String,
    pub description: Option<String>,
    pub profile_picture: Option<String>,
    pub keys: Key,
}

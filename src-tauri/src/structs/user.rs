use crate::structs::keys::one_time_prekey::OneTimePreKeyPublic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Key {
    pub ik_pub: [u8; 32],
    pub spk_pub: [u8; 32],
    pub opk_pub: Vec<OneTimePreKeyPublic>,
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
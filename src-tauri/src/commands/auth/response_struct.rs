use serde::{Serialize, Deserialize};
use crate::keys::one_time_prekey::OneTimePreKeyPublic;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: Tokens,
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
    pub opk_pub: Vec<OneTimePreKeyPublic>,
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
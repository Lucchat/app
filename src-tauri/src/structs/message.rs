use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub uuid: String,
    pub sender: String,
    pub receiver: String,
    pub nonce: [u8; 12],
    pub ciphertext: Vec<u8>,
    pub ratchet_pub: [u8; 32],
    pub message_index: u32,
    pub opk_used: Option<[u8; 32]>,
    pub ek_used: Option<[u8; 32]>,
    pub created_at: i64,
}
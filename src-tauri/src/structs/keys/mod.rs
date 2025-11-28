use serde::{Deserialize, Serialize};

pub mod chain_key;
pub mod ephemeral_key;
pub mod identity;
pub mod message_key;
pub mod one_time_prekey;
pub mod ratchet_key;
pub mod root_key;
pub mod session_key;
pub mod signed_prekey;

#[derive(Serialize, Deserialize)]
pub struct PrivateKeys {
    pub ik: identity::IdentityKey,
    pub spk: signed_prekey::SignedPreKey,
    pub opk: one_time_prekey::OneTimePreKeyGroup,
}

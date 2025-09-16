use serde::{Serialize, Deserialize};

pub mod identity;
pub mod one_time_prekey;
pub mod signed_prekey;

#[derive(Serialize, Deserialize)]
pub struct PrivateKeys {
    pub ik: identity::IdentityKey,
    pub spk: signed_prekey::SignedPreKey,
    pub opk: one_time_prekey::OneTimePreKeyGroup,
}
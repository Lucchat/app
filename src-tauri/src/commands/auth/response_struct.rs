use serde::{Deserialize, Serialize};
use crate::structs::user::User;

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

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
    pub password_check: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, Debug)]
pub struct LoginReply {
    pub success: bool,
    pub token: String,
}

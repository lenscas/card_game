use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, JsonSchema, Debug)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
    pub password_check: String,
}

#[derive(Deserialize, Serialize, JsonSchema, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}
#[derive(Serialize, JsonSchema, Debug)]
pub struct LoginReply {
    pub success: bool,
    pub token: String,
}

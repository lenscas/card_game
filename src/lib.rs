use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnBattle {
    pub success: bool,
    pub hand: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct TakeAction {
    pub play_card: usize,
}
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
#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
}

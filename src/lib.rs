use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnBattle {
    pub success: bool,
    pub hand: Vec<String>,
    pub small_runes: Vec<String>,
    pub enemy_small_runes: Vec<String>,
    pub player_hp: u64,
    pub enemy_hp: u64,
    pub enemy_hand_size: usize,
    pub mana: u64,
    pub enemy_mana: u64,
    pub hexa_runes: Vec<String>,
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

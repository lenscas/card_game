use serde::Serialize;

pub mod battle;
pub mod users;

#[derive(Serialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
}

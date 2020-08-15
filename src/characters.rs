use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterCreationResponse {
    pub id: i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CharacterList {
    pub characters: Vec<i64>,
}

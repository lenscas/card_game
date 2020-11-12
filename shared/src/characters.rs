use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CharacterCreationResponse {
    pub id: i64,
}
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct CharacterList {
    pub characters: Vec<i64>,
}

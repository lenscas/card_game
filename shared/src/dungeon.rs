use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum TileState {
    Seen(String),
    Empty,
    Hidden,
}
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct DungeonLayout {
    pub height: usize,
    pub widht: usize,
    pub player_at: crate::BasicVector<usize>,
    pub tiles: Vec<TileState>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum EventProcesed {
    Success(bool),
    Error,
    CurrentlyInBattle,
}

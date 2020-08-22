use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TileState {
    Seen(String),
    Empty,
    Hidden
}
#[derive(Serialize, Deserialize, Debug)]
pub struct DungeonLayout {
    pub height : usize,
    pub widht : usize,
    pub player_at : crate::BasicVector<usize>,
    pub tiles: Vec<TileState>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventProcesed {
    Success,
    Error,
    CurrentlyInBattle,
    
}
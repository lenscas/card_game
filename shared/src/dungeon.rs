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

#[derive(Copy, Clone, Deserialize, Serialize, Debug, PartialEq, JsonSchema)]
pub enum TileType {
    Empty,
    End,
    Start,
    Basic,
    Fight,
}
impl TileType {
    pub fn can_walk(&self) -> bool {
        match self {
            TileType::Empty => false,
            TileType::End | TileType::Start | TileType::Basic | TileType::Fight => true,
        }
    }
    pub fn can_leave_before_end(self) -> bool {
        match self {
            TileType::Start | TileType::Empty | TileType::Fight | TileType::Basic => false,
            TileType::End => true,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, JsonSchema)]
pub struct LandAction {
    pub image: String,
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq, JsonSchema)]
pub struct TileAction {
    pub tile_type: TileType,
    pub actions: Vec<LandAction>,
    pub can_leave: bool,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum EventProcesed {
    Success(Option<TileAction>),
    CurrentlyInAction(Option<TileAction>),
    Error,
}

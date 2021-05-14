use card_game_shared::dungeon::{TileAction, TileType};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};

use crate::{battle::Field, errors::ReturnError};
use card_game_shared::dungeon::LandAction;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct Tile {
    pub(crate) visited: bool,
    pub(crate) tile_type: TileType,
    pub(crate) has_been_processed: bool,
}

impl Tile {
    pub(crate) fn is_visible(&self) -> bool {
        self.visited || !self.tile_type.can_walk()
    }

    pub(crate) fn new(tile_type: TileType) -> Self {
        Self {
            visited: tile_type == TileType::Start,
            tile_type,
            has_been_processed: tile_type == TileType::Start,
        }
    }
    pub(crate) fn can_walk(&self) -> bool {
        self.tile_type.can_walk()
    }
    pub(crate) fn get_tile_actions(&self) -> TileAction {
        match self.tile_type {
            TileType::Empty | TileType::Start | TileType::Basic => TileAction {
                tile_type: self.tile_type,
                actions: Vec::new(),
                can_leave: self.tile_type.can_leave_before_end(),
            },
            TileType::End => TileAction {
                tile_type: self.tile_type,
                actions: vec![LandAction {
                    image: "/leave.png".into(),
                }],
                can_leave: true,
            },
            TileType::Fight => TileAction {
                tile_type: self.tile_type,
                actions: Vec::new(),
                can_leave: false,
            },
        }
    }
    pub(crate) async fn start_process(
        &mut self,
        user_id: i64,
        character_id: i64,
        con: &mut Transaction<'_, Postgres>,
    ) -> Result<TileAction, ReturnError> {
        match self.tile_type {
            TileType::Empty | TileType::Start | TileType::Basic => {
                self.has_been_processed = true;
            }
            TileType::End => {
                self.has_been_processed = true;
            }
            TileType::Fight => {
                let field = Field::new(user_id, character_id, con).await?;
                field.save(user_id, character_id, con).await?;
                self.has_been_processed = false;
            }
        }
        Ok(self.get_tile_actions())
    }
}

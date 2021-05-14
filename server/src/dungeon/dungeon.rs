use super::Tile;
use crate::errors::ReturnError;
use card_game_shared::dungeon::{TileAction, TileType};
use card_game_shared::BasicVector;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::{query, Executor};
use std::collections::HashSet;

use super::{connect_tiles, place_tile_at_random_location};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct Dungeon {
    pub(crate) height: usize,
    pub(crate) length: usize,
    pub(crate) tiles: Vec<Tile>,
    pub(crate) character_at: BasicVector<usize>,
}
impl Dungeon {
    pub(crate) async fn select_from_db_no_battle<'c, E>(
        user_id: i64,
        character_id: i64,
        con: E,
    ) -> Result<Option<Self>, ReturnError>
    where
        E: Executor<'c, Database = sqlx::postgres::Postgres>,
    {
        let dungeon = query!(
            "SELECT dungeon FROM characters WHERE user_id = $1 AND id=$2 AND characters.current_battle IS NULL",
            user_id,
            character_id
        )
        .fetch_optional(con)
        .await?.map(|d|d.dungeon).map(serde_json::from_value);

        match dungeon {
            Some(x) => Ok(Some(x?)),
            None => Ok(None),
        }
    }
    pub(crate) fn get_forced_actions_on_current_tile(&self) -> Option<TileAction> {
        self.tiles
            .get(card_game_shared::funcs::get_index_matrix(
                self.length as usize,
                &self.character_at,
            ))
            .and_then(|v| {
                ((!v.has_been_processed) && !v.tile_type.can_leave_before_end())
                    .then(|| v.get_tile_actions())
            })
    }
    pub(crate) async fn select_from_db<'c, E>(
        user_id: i64,
        character_id: i64,
        con: E,
    ) -> Result<Self, ReturnError>
    where
        E: Executor<'c, Database = sqlx::postgres::Postgres>,
    {
        let dungeon = query!(
            "SELECT dungeon FROM characters WHERE user_id = $1 AND id=$2",
            user_id,
            character_id
        )
        .fetch_one(con)
        .await?;

        Ok(serde_json::from_value(dungeon.dungeon)?)
    }
    pub(crate) fn to_shared(&self) -> card_game_shared::dungeon::DungeonLayout {
        card_game_shared::dungeon::DungeonLayout {
            height: self.height,
            widht: self.length,
            tiles: self
                .tiles
                .iter()
                .map(|v| match (v.is_visible(), &v.tile_type) {
                    (_, TileType::Empty) => card_game_shared::dungeon::TileState::Empty,
                    (true, _) => {
                        card_game_shared::dungeon::TileState::Seen(String::from("basic.png"))
                    }
                    (false, _) => card_game_shared::dungeon::TileState::Hidden,
                })
                .collect(),
            player_at: self.character_at.clone(),
        }
    }
    pub(crate) fn new(height: usize, length: usize) -> Self {
        let mut tiles = Vec::new();
        for _ in 0..length {
            for _ in 0..height {
                tiles.push(Tile::new(TileType::Empty))
            }
        }
        let mut chosen_tiles: HashSet<BasicVector<usize>> = Default::default();
        let mut rng = rand::thread_rng();
        let size = BasicVector {
            x: length,
            y: height,
        };
        let start_loc = place_tile_at_random_location(
            &mut rng,
            &mut chosen_tiles,
            &mut tiles,
            size.clone(),
            TileType::Start,
        );
        let end_loc = place_tile_at_random_location(
            &mut rng,
            &mut chosen_tiles,
            &mut tiles,
            size.clone(),
            TileType::End,
        );
        let count = connect_tiles(
            &mut rng,
            &mut chosen_tiles,
            &mut tiles,
            size.clone(),
            start_loc.clone(),
            end_loc,
        );
        let total_space = length * height;
        for _ in count + 2..total_space / 2 {
            let tile = if rng.gen() {
                TileType::Basic
            } else {
                TileType::Fight
            };
            place_tile_at_random_location(
                &mut rng,
                &mut chosen_tiles,
                &mut tiles,
                size.clone(),
                tile,
            );
        }

        Self {
            height,
            length,
            tiles,
            character_at: start_loc,
        }
    }
    pub(crate) fn try_move(&mut self, dir: BasicVector<isize>) -> Option<&mut Tile> {
        let old_character_location = self.character_at.clone();
        match (
            dir.x,
            dir.y,
            self.character_at.x == 0,
            self.character_at.x == self.length - 1,
            self.character_at.y == 0,
            self.character_at.y == self.height - 1,
        ) {
            (1, 0, _, false, _, _) => self.character_at.x += 1,
            (-1, 0, false, _, _, _) => self.character_at.x -= 1,
            (0, 1, _, _, _, false) => self.character_at.y += 1,
            (0, -1, _, _, false, _) => self.character_at.y -= 1,
            _ => return None,
        }
        let tile = self
            .tiles
            .get_mut(card_game_shared::funcs::get_index_matrix(
                self.length as usize,
                &self.character_at,
            ))
            .map(|v| if v.can_walk() { Some(v) } else { None });
        if tile.is_none() {
            self.character_at = old_character_location;
            return None;
        }
        tile.unwrap()
    }
    pub(crate) async fn save<'c, E>(
        self,
        user_id: i64,
        character_id: i64,
        con: E,
    ) -> Result<(), ReturnError>
    where
        E: Executor<'c, Database = sqlx::postgres::Postgres>,
    {
        query!(
            "UPDATE characters 
            SET dungeon=$1 
            WHERE user_id = $2 
            AND id = $3",
            serde_json::to_value(self)?,
            user_id,
            character_id
        )
        .execute(con)
        .await?;
        Ok(())
    }
}

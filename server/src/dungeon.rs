use crate::errors::ReturnError;
use card_game_shared::BasicVector;
use rand::{prelude::ThreadRng, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{query, Executor};
use std::collections::HashSet;

fn get_random_tile_loc(rng: &mut ThreadRng, width: usize, height: usize) -> BasicVector<usize> {
    BasicVector {
        x: rng.gen_range(0..(width - 1)),
        y: rng.gen_range(0..(height - 1)),
    }
}

fn place_tile_at_random_location(
    rng: &mut ThreadRng,
    filled_in_locations: &mut HashSet<BasicVector<usize>>,
    tiles: &mut Vec<Tile>,
    size: BasicVector<usize>,
    tile_type: TileType,
) -> BasicVector<usize> {
    loop {
        let tile_location = get_random_tile_loc(rng, size.x, size.y);
        let index_for_tile = card_game_shared::funcs::get_index_matrix(size.x, &tile_location);
        let current = &tiles[index_for_tile];
        if current.tile_type == TileType::Empty {
            tiles[index_for_tile] = Tile::new(tile_type);
            filled_in_locations.insert(tile_location.clone());
            return tile_location;
        }
    }
}

fn get_randomly_closer(
    rng: &mut ThreadRng,
    current: usize,
    target: usize,
    max_size: usize,
) -> usize {
    if rng.gen_range(0.0..1.0) < 0.4 {
        if current < max_size - 1 {
            current + 1
        } else {
            current
        }
    } else if current < target {
        current + 1
    } else if current > target {
        current - 1
    } else if rng.gen_range(0. ..1.) < 0.3 {
        current
    } else if rng.gen() {
        if current < max_size - 1 {
            current + 1
        } else {
            current
        }
    } else if current > 0 {
        current - 1
    } else {
        current
    }
}

fn connect_tiles(
    rng: &mut ThreadRng,
    filled_in_locations: &mut HashSet<BasicVector<usize>>,
    tiles: &mut Vec<Tile>,
    size: BasicVector<usize>,
    start: BasicVector<usize>,
    mut end: BasicVector<usize>,
) -> usize {
    let mut current = start;
    let mut count = 0;
    while current != end {
        let new_current = if rng.gen() {
            BasicVector {
                x: get_randomly_closer(rng, current.x, end.x, size.x),
                y: current.y,
            }
        } else {
            BasicVector {
                x: current.x,
                y: get_randomly_closer(rng, current.y, end.y, size.y),
            }
        };

        if new_current != end {
            let index = card_game_shared::funcs::get_index_matrix(size.x, &new_current);
            let old_tile = &tiles[index];
            if old_tile.tile_type == TileType::Empty {
                tiles[index] = Tile::new(if rng.gen() {
                    TileType::Basic
                } else {
                    TileType::Fight
                });
                filled_in_locations.insert(new_current.clone());
                count += 1;
            }
        } else {
            return count;
        }

        end = dbg!(end);
        current = dbg!(new_current);
    }
    count
}

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub(crate) enum TileType {
    Empty,
    End,
    Start,
    Basic,
    Fight,
}
impl TileType {
    pub(crate) fn can_walk(&self) -> bool {
        match self {
            TileType::Empty => false,
            TileType::End | TileType::Start | TileType::Basic | TileType::Fight => true,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub(crate) struct Tile {
    pub(crate) visited: bool,
    pub(crate) tile_type: TileType,
}

impl Tile {
    pub(crate) fn is_visible(&self) -> bool {
        self.visited || !self.tile_type.can_walk()
    }
    pub(crate) fn new(tile_type: TileType) -> Self {
        Self {
            visited: tile_type == TileType::Start,
            tile_type,
        }
    }
    pub(crate) fn can_walk(&self) -> bool {
        self.tile_type.can_walk()
    }
}
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
    #[must_use]
    pub(crate) fn try_move(&mut self, dir: BasicVector<isize>) -> Option<bool> {
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
        self.tiles
            .get_mut(card_game_shared::funcs::get_index_matrix(
                self.length as usize,
                &self.character_at,
            ))
            .and_then(|v| if v.can_walk() { Some(v) } else { None })
            .map(|v| {
                let already_visited = v.visited;
                v.visited = true;
                Some((!already_visited) && v.tile_type == TileType::Fight)
            })
            .unwrap_or_else(|| {
                self.character_at = old_character_location;
                None
            })
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

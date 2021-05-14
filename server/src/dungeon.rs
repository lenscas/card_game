mod dungeon;
mod tile;

use card_game_shared::{dungeon::TileType, BasicVector};
use rand::{prelude::ThreadRng, Rng};
use std::collections::HashSet;

pub(crate) use dungeon::Dungeon;
pub(crate) use tile::Tile;

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

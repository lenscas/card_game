use serde::{Deserialize, Serialize};

pub mod battle;
pub mod battle_log;
pub mod characters;
pub mod dungeon;
pub mod funcs;
pub mod users;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorMessage {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct BasicVector<T> {
    pub x: T,
    pub y: T,
}

impl<T> BasicVector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[cfg(feature = "client")]
impl From<BasicVector<i64>> for quicksilver::geom::Vector {
    fn from(v: BasicVector<i64>) -> quicksilver::geom::Vector {
        quicksilver::geom::Vector {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}
#[cfg(feature = "client")]
impl From<BasicVector<u64>> for quicksilver::geom::Vector {
    fn from(v: BasicVector<u64>) -> quicksilver::geom::Vector {
        quicksilver::geom::Vector {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}

use std::default;

use super::*;

pub const TILE_SIZE: f32 = 8.0;

#[derive(Default, Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    DownStairs,
    UpStairs,

    Floor,
    Wall,
    Door,

    #[default]
    DeepWater,
    Water,
    Sand,
    Dirt,
    Grass,

    Mountain,
    Snow,
}

impl TileType {
    pub fn is_walkable(&self) -> bool {
        use TileType::*;
        matches!(self, Floor | DownStairs | UpStairs | Water | Sand | Grass | Dirt | Snow)
    }

    pub fn is_opaque(&self) -> bool {
        matches!(self, TileType::Wall)
    }

    pub fn cost(&self) -> f32 {
        // match self {
        //     TileType::Road => 0.8,
        //     TileType::Grass => 1.1,
        //     TileType::ShallowWater => 1.2,
        //     _ => 1.0,
        // }
        1.0
    }

    pub fn is_wall(&self) -> bool {
        matches!(self, TileType::Wall)
    }

    pub fn is_floor(&self) -> bool {
        matches!(self, TileType::Floor)
    }
}

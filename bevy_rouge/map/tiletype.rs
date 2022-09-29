use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
    UpStairs,
}

impl TileType {
    pub fn walkable(&self) -> bool {
        matches!(self, TileType::Floor | TileType::DownStairs | TileType::UpStairs)
    }

    pub fn opaque(&self) -> bool { matches!(self, TileType::Wall) }

    pub fn cost(&self) -> f32 {
        // match self {
        //     TileType::Road => 0.8,
        //     TileType::Grass => 1.1,
        //     TileType::ShallowWater => 1.2,
        //     _ => 1.0,
        // }
        1.0
    }
}

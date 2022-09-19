use super::*;

pub const TILE_SIZE: f32 = 16.0;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    DownStairs,
    UpStairs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameTile {
    pub flags: TileFlags,
    pub tile_type: TileType,
}

impl GameTile {
    pub fn new(tile_type: TileType) -> Self {
        Self { flags: TileFlags::empty(), tile_type }
    }

    pub fn walkable(&self) -> bool {
        matches!(self.tile_type, TileType::Floor | TileType::DownStairs | TileType::UpStairs)
            && !self.flags.contains(TileFlags::BLOCKS_MOVEMENT)
    }

    pub fn is_opaque(&self) -> bool {
        matches!(self.tile_type, TileType::Wall)
            && !self.flags.contains(TileFlags::BLOCKS_VISION)
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
}

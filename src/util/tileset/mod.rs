use crate::prelude::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::{collections::HashMap, fmt::Debug, path::PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TileSets {
    Ascii,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

lazy_static! {
    pub static ref TILESETS: Mutex<HashMap<TileSets, Tileset>> = Mutex::new({
        let mut map = HashMap::new();
        map.insert(TileSets::Ascii, ascii_tileset());
        map
    });
}

pub fn add_tileset(set: TileSets, tileset: Tileset) {
    let tiles = &mut *TILESETS.lock();
    tiles.insert(set, tileset);
}

pub fn get_tile_index(set: TileSets, symbol: GameSymbol) -> usize {
    let tileset = TILESETS.lock();
    match tileset.get(&set) {
        Some(set) => *set.cellsym_map.get(&symbol).unwrap(),
        None => 0,
    }
}

pub fn get_wall_tile(set: TileSets) -> usize {
    let mut rng = RandomNumberGenerator::new();
    match set {
        TileSets::Ascii => {
            if rng.range(0, 100) < 30 {
                65
            } else {
                get_tile_index(set, GameSymbol::Wall)
            }
        }
    }
}

pub fn get_floor_tile(set: TileSets) -> usize {
    get_tile_index(set, GameSymbol::Floor)
    // let mut rng = RandomNumberGenerator::new();
    // match set {
    //     TileSets::Ascii => {
    //         if rng.range(0, 100) < 30 {
    //             rng.range(70, 75)
    //         } else {
    //             get_tile_index(set, GameSymbol::Floor)
    //         }
    //     }
    // }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// TileSet Info
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl From<(u32, u32)> for Size {
    fn from((w, h): (u32, u32)) -> Self {
        Self { w, h }
    }
}

pub trait Symbol: Copy + Clone + Eq + PartialEq + std::hash::Hash + Debug {}

/// Data describing a tileset that can be loaded from an image on a file system.
pub struct Tileset {
    /// Path to the tile image.
    pub image_path: PathBuf,
    /// Pixel width and height of tiles in the tileset.
    pub tile_size: Size,
    /// Pixel offset of the top-left tile in the tileset.
    pub tile_start: Point,
    /// Number of pixels between tiles across.
    pub tile_gap: Size,
    /// Map of symbols to tile positions in the tile image.
    pub cellsym_map: HashMap<GameSymbol, usize>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameSymbol {
    Floor,
    DungeonFloor,
    Wall,
    DownStairs,
    UpStairs,

    Player,
    Enemy,
}

impl Symbol for GameSymbol {}

pub fn ascii_tileset() -> Tileset {
    let mut cellsym_map: HashMap<GameSymbol, usize> = HashMap::new();
    {
        use GameSymbol::*;

        ///////////////////////////////
        // Floors
        ///////////////////////////////
        cellsym_map.insert(Floor, 237);
        cellsym_map.insert(DungeonFloor, 70);

        ///////////////////////////////
        // Walls
        ///////////////////////////////
        cellsym_map.insert(Wall, 64);

        cellsym_map.insert(UpStairs, 290);
        cellsym_map.insert(DownStairs, 291);

        cellsym_map.insert(Player, 225);
        cellsym_map.insert(Enemy, 262);
    }

    Tileset {
        cellsym_map,
        tile_gap: (0, 0).into(),
        tile_start: (0, 0).into(),
        tile_size: (16, 16).into(),
        image_path: PathBuf::from("resources/textures/Dungeon-Tileset.png"),
    }
}

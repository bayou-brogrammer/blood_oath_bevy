use crate::prelude::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::{collections::HashMap, fmt::Debug, path::PathBuf};

mod gamesym;
pub use gamesym::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

lazy_static! {
    pub static ref GAME_TILESET: Mutex<Tileset<GameSymbol>> =
        Mutex::new(new_tilset(one_bit_kenny_tileset_info()));
}

pub fn new_tilset(tileset_info: TilesetInfo<GameSymbol>) -> Tileset<GameSymbol> {
    Tileset::new(tileset_info)
}

pub fn get_tile_index(symbol: GameSymbol) -> usize {
    let tileset = GAME_TILESET.lock();
    *tileset.cellsym_map.get(&symbol).unwrap_or(&0)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// TileSet Info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Position of a tile in a tile image.
pub type TileIndex = (i32, i32);

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

pub trait Symbol: Copy + Clone + Eq + PartialEq + std::hash::Hash + Debug {
    fn text_fallback(self) -> char;
}

/// Data describing a tileset that can be loaded from an image on a file system.
pub struct TilesetInfo<Y: Symbol> {
    /// Path to the tile image.
    pub image_path: PathBuf,
    /// Pixel width and height of tiles in the tileset.
    pub tile_size: Size,
    /// Pixel offset of the top-left tile in the tileset.
    pub tile_start: Point,
    /// Number of pixels between tiles across.
    pub tile_gap: Size,
    // /// Map of characters to glyph positions in the tile image.
    // pub font_map: HashMap<char, TileIndex>,
    /// Map of symbols to tile positions in the tile image.
    pub symbol_map: HashMap<Y, TileIndex>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// TileSet
////////////////////////////////////////////////////////////////////////////////////////////////////

/// A set of symbols mapped to positions in a tile image.
///
/// Used by TileGrid to measure out and render its contents to its buffer.
pub struct Tileset<Y: Symbol> {
    tile_size: Size,
    cellsym_map: HashMap<Y, usize>,
}

impl<Y: Symbol> Tileset<Y> {
    pub fn new(tileset_info: TilesetInfo<Y>) -> Self {
        assert!(!tileset_info.symbol_map.is_empty(), "at least one tile must be mapped");
        assert!(tileset_info.tile_start.x >= 0 && tileset_info.tile_start.y >= 0);

        let tile_w = tileset_info.tile_size.w;

        let mut cellsym_map: HashMap<Y, usize> = HashMap::new();
        for (sym, (x, y)) in tileset_info.symbol_map {
            let idx = (y as usize * tile_w as usize) + x as usize;
            cellsym_map.insert(sym, idx);
        }

        Self { tile_size: tileset_info.tile_size, cellsym_map }
    }
}

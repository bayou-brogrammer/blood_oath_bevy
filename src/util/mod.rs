use crate::prelude::*;

pub mod tileset;
pub use tileset::*;

pub fn get_sprite(index: usize) -> TextureAtlasSprite {
    let mut sprite = TextureAtlasSprite::new(index);
    // sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
    sprite
}

/// Build a properly sized [`TextureAtlasSprite`] with the given index
pub fn get_sprite_with_color(index: usize, color: Color) -> TextureAtlasSprite {
    let mut sprite = TextureAtlasSprite::new(index);
    // sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
    sprite.color = color;
    sprite
}

/// Takes a Point and returns World Coords
pub fn pt_spritecoords(pos: Point) -> (f32, f32) {
    let x: f32 = pos.x as f32 * TILE_SIZE + (TILE_SIZE / 2.0);
    let y: f32 = pos.y as f32 * TILE_SIZE + (TILE_SIZE / 2.0);
    (x, y)
}

pub trait FromTilePos {
    fn from_tilepos(tilepos: TilePos) -> Self;
}

impl FromTilePos for Point {
    fn from_tilepos(tilepos: TilePos) -> Point {
        Point::new(tilepos.x, tilepos.y)
    }
}

use crate::prelude::*;

mod macros;
pub use macros::*;

pub fn get_sprite(index: usize) -> TextureAtlasSprite {
    TextureAtlasSprite::new(index)
}

/// Build a properly sized [`TextureAtlasSprite`] with the given index
pub fn get_sprite_with_color(index: usize, color: Color) -> TextureAtlasSprite {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;
    sprite
}

/// Takes a Point and returns World Coords
pub fn pt_spritecoords(pos: Point) -> (f32, f32) {
    let x: f32 = pos.x as f32 * TILE_SIZE + (TILE_SIZE / 2.);
    let y: f32 = pos.y as f32 * TILE_SIZE + (TILE_SIZE / 2.);
    (x, y)
}

///////////////////
/// To Point
///////////////////
use grid_2d::{Coord, Size};

pub trait ToPoint {
    fn to_point(&self) -> Point;
}

impl ToPoint for TilePos {
    fn to_point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl ToPoint for Coord {
    fn to_point(&self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl ToPoint for Size {
    fn to_point(&self) -> Point {
        self.to_coord().unwrap().to_point()
    }
}

///////////////////
/// To Coord
///////////////////
pub trait ToCoord {
    fn to_coord(&self) -> Coord;
}

impl ToCoord for Point {
    fn to_coord(&self) -> Coord {
        Coord::new(self.x, self.y)
    }
}

use crate::prelude::*;
use std::collections::HashSet;

mod glyph;
mod pos;
mod stat;
mod tags;
mod turn;

pub use glyph::*;
pub use pos::*;
pub use stat::*;
pub use tags::*;
pub use turn::*;

#[derive(Component)]
pub struct CameraFollow(pub Entity);

#[derive(Default, Component)]
pub struct FieldOfView {
    pub radius: i32,
    pub is_dirty: bool,
    pub visible_tiles: HashSet<Point>,
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self { radius, is_dirty: true, visible_tiles: HashSet::new() }
    }
}

use crate::prelude::*;
use std::collections::HashSet;

mod pos;
mod tags;
mod turn;

pub use pos::*;
pub use tags::*;
pub use turn::*;

#[derive(Component)]
pub struct CameraFollow(pub Entity);

#[derive(Default, Component)]
pub struct FieldOfView {
    pub radius: u32,
    pub visible_tiles: HashSet<Point>,
}

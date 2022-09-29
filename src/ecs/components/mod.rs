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
    pub radius: i32,
    pub visible_tiles: HashSet<Point>,
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct LightSource {
    pub color: Color,
    pub range: i32,
}

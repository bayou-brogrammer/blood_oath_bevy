use crate::dungeon_crawl::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}
impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self { visible_tiles: HashSet::new(), radius, is_dirty: true }
    }
}

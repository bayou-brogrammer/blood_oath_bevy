use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

mod bitgrid;
mod spatial;
mod tiletype;

pub use bitgrid::*;
pub use spatial::*;
pub use tiletype::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub depth: i32,
    pub width: i32,
    pub height: i32,
    pub name: String,
    pub visible: BitGrid,
    pub revealed: BitGrid,
    pub tiles: Vec<TileType>,
    pub blocked: Vec<(bool, bool)>,
    pub view_blocked: HashSet<usize>,

    // #[serde(skip_serializing)]
    // #[serde(skip_deserializing)]
    // #[serde(default = "default_theme")]
    // pub theme: Box<dyn MapTheme>,
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    tile_content: Vec<Vec<(Entity, bool)>>,
}

impl Clone for Map {
    fn clone(&self) -> Self {
        Self {
            depth: self.depth,
            width: self.width,
            height: self.height,
            name: self.name.clone(),
            tiles: self.tiles.clone(),
            // theme: DungeonTheme::build(),
            visible: self.visible.clone(),
            blocked: self.blocked.clone(),
            revealed: self.revealed.clone(),
            view_blocked: self.view_blocked.clone(),
            tile_content: self.tile_content.clone(),
        }
    }
}

// fn default_theme() -> Box<dyn MapTheme> {
//     DungeonTheme::build()
// }

impl Map {
    /// Generates an empty map, consisting entirely of solid walls
    pub fn new<S: ToString>(width: i32, height: i32, depth: i32, name: S) -> Map {
        let map_tile_count = (width * height) as usize;

        Map {
            depth,
            width,
            height,
            name: name.to_string(),
            view_blocked: HashSet::new(),
            // theme: crate::DungeonTheme::build(),
            visible: BitGrid::new(width, height),
            revealed: BitGrid::new(width, height),
            tiles: vec![TileType::Wall; map_tile_count],
            blocked: vec![(false, false); map_tile_count],
            tile_content: vec![Vec::new(); map_tile_count],
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    // checks if it is physically possible (ie no wall or physical object)
    pub fn can_enter_tile(&self, pt: Point) -> bool {
        let idx = self.point2d_to_index(pt);
        self.in_bounds(pt) && !self.is_blocked(idx)
    }

    pub fn try_idx(&self, pt: Point) -> Option<usize> {
        if !self.in_bounds(pt) {
            None
        } else {
            Some(self.point2d_to_index(pt))
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y > 0 && pos.y < self.height as i32
    }
}

#[rustfmt::skip]
impl BaseMap for Map {
    fn is_opaque(&self, idx:usize) -> bool {
        if idx > 0 && idx < self.tiles.len() {
            self.tiles[idx].opaque() || self.view_blocked.contains(&idx)
        } else {
            true
        }
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);
        let tt = self.tiles[idx];

        // Cardinals
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) { exits.push((idx, tt.cost())) }

        // Diagonals
        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) { exits.push((idx, tt.cost())) }

        exits
    }

    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

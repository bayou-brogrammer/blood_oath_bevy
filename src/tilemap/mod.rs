use bitflags::bitflags;

use crate::prelude::*;
use grid_2d::Grid;
use grid_2d::Size;

mod bitgrid;
mod map_builder;
mod spatial;
mod themes;
mod tile;

pub use bitgrid::*;
pub use map_builder::*;
pub use spatial::*;
pub use themes::*;
pub use tile::*;

bitflags! {
    #[derive(Default, Serialize, Deserialize)]
    pub struct TileFlags: u32{
        const BLOCKS_MOVEMENT = 1 << 0;
        const BLOCKS_VISION = 1 << 2;
        const IN_VIEW = 1 << 3;
        const EXPLORED = 1 << 4;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMap {
    pub depth: i32,
    pub size: Size,
    pub name: String,

    pub visible: BitGrid,
    pub revealed: BitGrid,
    pub opaque: BitGrid,
    pub blocked: BitGrid,
    pub tiles: Grid<TileType>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<(Entity, bool, bool)>>,
}

impl TileMap {
    pub fn new<S: ToString>(size: Size, depth: i32, name: S) -> Self {
        let map_tile_count = size.count();

        Self {
            size,
            depth,
            name: name.to_string(),

            visible: BitGrid::new(size),
            revealed: BitGrid::new(size),

            opaque: BitGrid::new(size),
            blocked: BitGrid::new(size),
            tile_content: vec![Vec::new(); map_tile_count],
            tiles: Grid::new_fn(size, |_| TileType::Wall),
        }
    }

    pub fn width(&self) -> i32 {
        self.size.width() as i32
    }

    pub fn height(&self) -> i32 {
        self.size.height() as i32
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        self.coord_to_index(Coord::new(x, y))
    }

    /// Convert a Point (x/y) to an array index. Defaults to an index based on an array
    /// strided X first.
    pub fn coord_to_index(&self, coord: Coord) -> usize {
        let bounds = self.dimensions();
        ((coord.y * bounds.x) + coord.x)
            .try_into()
            .expect("Not a valid usize. Did something go negative?")
    }

    /// Convert an array index to a point. Defaults to an index based on an array
    /// strided X first.
    pub fn index_to_coord(&self, idx: usize) -> Coord {
        let bounds = self.dimensions();
        let w: usize =
            bounds.x.try_into().expect("Not a valid usize. Did something go negative?");
        Coord::new((idx % w) as i32, (idx / w) as i32)
    }

    pub fn in_bounds(&self, coord: Coord) -> bool {
        self.size.is_valid(coord)
    }

    #[allow(dead_code)]
    #[inline]
    pub fn try_idx(&self, coord: Coord) -> Option<usize> {
        if !self.in_bounds(coord) {
            None
        } else {
            Some(self.coord_to_index(coord))
        }
    }

    pub fn clear(&mut self) {
        self.clear_blocked();
        self.clear_opaque();
        self.tile_content.iter_mut().for_each(|v| v.clear());
    }

    // checks if it is physically possible (ie no wall or physical object)
    pub fn can_enter_tile(&self, coord: Coord) -> bool {
        let idx = self.coord_to_index(coord);
        self.size.is_valid(coord) && self.in_bounds(coord) && !self.is_blocked(idx)
    }

    fn valid_exit(&self, loc: Coord, delta: Coord) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.coord_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for TileMap {
    fn dimensions(&self) -> Point {
        self.size.to_point()
    }
}

#[rustfmt::skip]
impl BaseMap for TileMap {
    fn is_opaque(&self, idx:usize) -> bool {
        if idx > 0 && idx < self.tiles.len() {
            self.is_opaque(idx)
        } else {
            true
        }
    }

    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_coord(idx);
        let tt = &self.tiles.get_index_checked(idx);

        // Cardinals
        if let Some(idx) = self.valid_exit(location, Coord::new(-1, 0)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Coord::new(1, 0)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Coord::new(0, -1)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Coord::new(0, 1)) { exits.push((idx, tt.cost())) }

        // Diagonals
        if let Some(idx) = self.valid_exit(location, Coord::new(-1, -1)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Coord::new(1, -1)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Coord::new(-1, 1)) { exits.push((idx, tt.cost())) }
        if let Some(idx) = self.valid_exit(location, Coord::new(1, 1)) { exits.push((idx, tt.cost())) }

        exits
    }
}

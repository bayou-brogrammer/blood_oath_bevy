use crate::prelude::*;

mod bitgrid;
mod builders;
mod spatial;
mod tile;

use bitflags::bitflags;
pub use bitgrid::*;
pub use builders::*;
pub use spatial::*;
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TileMap {
    pub depth: i32,
    pub width: i32,
    pub height: i32,
    pub name: String,

    pub visible: BitGrid,
    pub revealed: BitGrid,
    pub tiles: Vec<GameTile>,

    pub light: Vec<Color>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<(Entity, bool, bool)>>,
}

impl TileMap {
    pub fn new<S: ToString>(width: i32, height: i32, depth: i32, name: S) -> Self {
        let size = TilemapSize { x: width as u32, y: height as u32 };
        let map_tile_count = size.count();

        Self {
            depth,
            width,
            height,
            name: name.to_string(),

            visible: BitGrid::new(width, height),
            revealed: BitGrid::new(width, height),

            // opaque: BitGrid::new(width, height),
            // blocked: BitGrid::new(width, height),
            tile_content: vec![Vec::new(); map_tile_count],
            tiles: vec![GameTile::new(TileType::Wall); map_tile_count],
            light: vec![Color::BLACK; map_tile_count],
        }
    }

    #[inline]
    fn tiletype(&self, pt: Point) -> TileType {
        self.tiles[self.point2d_to_index(pt)].tile_type
    }

    #[allow(dead_code)]
    #[inline]
    pub fn try_idx(&self, pt: Point) -> Option<usize> {
        if !self.in_bounds(pt) {
            None
        } else {
            Some(self.point2d_to_index(pt))
        }
    }

    // checks if it is physically possible (ie no wall or physical object)
    pub fn can_enter_tile(&self, pt: Point) -> bool {
        let idx = self.point2d_to_index(pt);
        self.in_bounds(pt) && !self.is_blocked(idx)
    }

    pub fn texture_for_tiletype(&self, tile_pos: TilePos, rng: &RandomNumbers) -> TileTexture {
        let tile_type = self.tiletype(tile_pos.to_point());
        let index = match tile_type {
            TileType::Floor => get_floor_tile(TileSets::Ascii),
            TileType::Wall => get_wall_tile(TileSets::Ascii),
            TileType::DownStairs => todo!(),
            TileType::UpStairs => todo!(),
            _ => 0,
        };

        TileTexture(index as u32)
    }

    pub fn spawn(
        &self,
        commands: &mut Commands,
        rng: &RandomNumbers,
        texture_handle: Handle<Image>,
    ) -> Entity {
        let tilemap_entity = commands.spawn().id();
        let size = TilemapSize { x: self.width as u32, y: self.height as u32 };
        let mut tile_storage = TileStorage::empty(size);

        for x in 0..self.width as u32 {
            for y in 0..self.height as u32 {
                let tile_pos = TilePos { x, y };
                let tile_texture = self.texture_for_tiletype(tile_pos, rng);

                let tile_entity = commands
                    .spawn()
                    .insert_bundle(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture: tile_texture,
                        visible: TileVisible(false),
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&tile_pos, Some(tile_entity));
            }
        }

        let tile_size = TilemapTileSize { x: TILE_SIZE, y: TILE_SIZE };

        commands.entity(tilemap_entity).insert_bundle(TilemapBundle {
            size,
            tile_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            grid_size: TilemapGridSize { x: TILE_SIZE, y: TILE_SIZE },
            ..Default::default()
        });

        tilemap_entity
    }

    pub fn wall_or_oob(&self, x: i32, y: i32) -> bool {
        self.in_bounds(Point::new(x, y)) && self.tiletype(Point::new(x, y)) == TileType::Wall
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

impl Algorithm2D for TileMap {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    fn in_bounds(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
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
        let location = self.index_to_point2d(idx);
        let tt = &self.tiles[idx];

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
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MapBuilderPlugin);
    }
}

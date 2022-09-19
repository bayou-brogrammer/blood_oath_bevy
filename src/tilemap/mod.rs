use crate::prelude::*;
use std::collections::HashSet;

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
    /// Stored left-to-right, top-to-bottom
    pub depth: i32,
    pub width: i32,
    pub height: i32,
    pub name: String,
    pub visible: BitGrid,
    pub revealed: BitGrid,
    pub tiles: Vec<GameTile>,
    pub blocked: Vec<(bool, bool)>,
    pub view_blocked: HashSet<usize>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<(Entity, bool)>>,
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
            view_blocked: HashSet::new(),
            visible: BitGrid::new(width, height),
            revealed: BitGrid::new(width, height),
            blocked: vec![(false, false); map_tile_count],
            tile_content: vec![Vec::new(); map_tile_count],
            tiles: vec![GameTile::new(TileType::Wall); map_tile_count],
        }
    }

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

    pub fn texture_for_tiletype(&self, tile_pos: TilePos) -> TileTexture {
        let tile_type = self.tiletype(Point::from_tilepos(tile_pos));
        let index = match tile_type {
            TileType::Floor => tileset::get_tile_index(GameSymbol::Floor),
            TileType::Wall => tileset::get_tile_index(self.wall_sym(tile_pos)),
            TileType::DownStairs => todo!(),
            TileType::UpStairs => todo!(),
        };

        TileTexture(index as u32)
    }

    pub fn spawn(&self, commands: &mut Commands, texture_handle: Handle<Image>) -> Entity {
        let tilemap_entity = commands.spawn().id();
        let size = TilemapSize { x: self.width as u32, y: self.height as u32 };
        let mut tile_storage = TileStorage::empty(size);

        for x in 0..self.width as u32 {
            for y in 0..self.height as u32 {
                let tile_pos = TilePos { x, y };
                let tile_texture = self.texture_for_tiletype(tile_pos);
                let tile_entity = commands
                    .spawn()
                    .insert_bundle(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(tilemap_entity),
                        texture: tile_texture,
                        visible: TileVisible(true),
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

    #[allow(clippy::many_single_char_names)]
    fn wall_sym(&self, tile_pos: TilePos) -> GameSymbol {
        let x = tile_pos.x as i32;
        let y = tile_pos.y as i32;

        let n = self.wall_or_oob(x, y - 1);
        let s = self.wall_or_oob(x, y + 1);
        let e = self.wall_or_oob(x + 1, y);
        let w = self.wall_or_oob(x - 1, y);
        let ne = self.wall_or_oob(x + 1, y - 1);
        let nw = self.wall_or_oob(x - 1, y - 1);
        let se = self.wall_or_oob(x + 1, y + 1);
        let sw = self.wall_or_oob(x - 1, y + 1);

        // Extend wall stems in a direction if it has a wall,
        // and at least one of its cardinal/diagonal adjacent tiles is not a wall.
        let mut mask: u8 = 0;

        if n && (!ne || !nw || !e || !w) {
            mask += 1;
        }
        if s && (!se || !sw || !e || !w) {
            mask += 2;
        }
        if w && (!nw || !sw || !n || !s) {
            mask += 4;
        }
        if e && (!ne || !se || !n || !s) {
            mask += 8;
        }

        match mask {
            0 => GameSymbol::WallPillar,
            1 => GameSymbol::WallN,
            2 => GameSymbol::WallS,
            3 => GameSymbol::WallNs,
            4 => GameSymbol::WallW,
            5 => GameSymbol::WallNw,
            6 => GameSymbol::WallSw,
            7 => GameSymbol::WallNsw,
            8 => GameSymbol::WallE,
            9 => GameSymbol::WallNe,
            10 => GameSymbol::WallEs,
            11 => GameSymbol::WallNes,
            12 => GameSymbol::WallEw,
            13 => GameSymbol::WallNew,
            14 => GameSymbol::WallEsw,
            15 => GameSymbol::WallNesw,
            _ => GameSymbol::WallOther,
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
            self.tiles[idx].is_opaque() || self.view_blocked.contains(&idx)
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

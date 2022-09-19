use crate::prelude::*;

mod rooms;
pub use rooms::*;

trait MapArchitect {
    fn build(&mut self, width: i32, height: i32, depth: i32) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: TileMap,
    rooms: Vec<Rect>,
    pub player_start: Position,
    pub spawn_list: Vec<Position>,
}

impl MapBuilder {
    pub fn new(width: i32, height: i32) -> Self {
        #[allow(clippy::match_single_binding)]
        let mut architect: Box<dyn MapArchitect> = match crate::rng::range(0, 1) {
            _ => Box::new(RoomsArchitect {}),
        };

        architect.build(width, height, 0)
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| t.tile_type = tile);
    }

    fn build_random_rooms(&mut self) {
        const NUM_ROOMS: usize = 20;

        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                crate::rng::range(1, self.map.width as u32 - 10),
                crate::rng::range(1, self.map.height as u32 - 10),
                crate::rng::range(2, 10),
                crate::rng::range(2, 10),
            );
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < self.map.width && p.y > 0 && p.y < self.map.height {
                        let idx = self.map.point2d_to_index(p);
                        self.map.tiles[idx].tile_type = TileType::Floor;
                    }
                });

                self.rooms.push(room)
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize].tile_type = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize].tile_type = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if crate::rng::range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////
// Builder
//////////////////////////////////////////////////////////////////////////////////////////

fn setup_tilemap(mut commands: Commands, textures: Res<TextureAssets>) {
    println!("Setting up tilemap");

    let mb = MapBuilder::new(80, 50);
    mb.map.spawn(&mut commands, textures.tileset.clone());

    commands.insert_resource(mb.map.clone());
    commands.insert_resource(mb);
    commands.insert_resource(NextState(GameState::InGame))
}

pub struct MapBuilderPlugin;
impl Plugin for MapBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Setup, setup_tilemap);
    }
}

use crate::prelude::*;
use std::collections::HashMap;

mod builders;
mod common;
mod meta_builders;
mod random;
mod themes;

pub use builders::*;
pub use common::*;
pub use meta_builders::*;
pub use random::*;
pub use themes::*;

////////////////////////////////////////////////////////////////////////////////
// Map Builder Traits
////////////////////////////////////////////////////////////////////////////////

pub trait InitialMapArchitect {
    fn build_map(&mut self, map_builder: &mut MapBuilder, rng: &mut RandomNumbers);
}

pub trait MapArchitect {
    fn build_map(&mut self, map_builder: &mut MapBuilder, rng: &mut RandomNumbers);
}

pub trait MapTheme: std::fmt::Debug + Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> Option<Glyph>;
}

////////////////////////////////////////////////////////////////////////////////

pub struct BuilderChain {
    pub map_builder: MapBuilder,
    builders: Vec<Box<dyn MapArchitect>>,
    starter: Option<Box<dyn InitialMapArchitect>>,
}

impl BuilderChain {
    pub fn new<S: ToString>(level: i32, width: i32, height: i32, name: S) -> BuilderChain {
        BuilderChain {
            starter: None,
            builders: Vec::new(),
            map_builder: MapBuilder::new(level, width, height, name),
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapArchitect>) {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder."),
        };
    }

    pub fn with(&mut self, metabuilder: Box<dyn MapArchitect>) {
        self.builders.push(metabuilder);
    }

    pub fn build_map(&mut self, rng: &mut RandomNumbers) {
        match &mut self.starter {
            None => panic!("Cannot run a map builder chain without a starting build system"),
            Some(starter) => {
                // Build the starting map
                starter.build_map(&mut self.map_builder, rng);
            }
        }

        // Build additional layers in turn
        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(&mut self.map_builder, rng);
        }
    }
}

#[derive(Debug, Clone)]
pub struct MapBuilder {
    pub map: Map,
    pub width: i32,
    pub height: i32,
    pub amulet_start: Point,
    pub rooms: Option<Vec<Rect>>,
    pub spawn_list: Vec<(usize, String)>,
    pub corridors: Option<Vec<Vec<usize>>>,
    pub starting_position: Option<Point>,
}

impl MapBuilder {
    pub fn new<S: ToString>(new_depth: i32, width: i32, height: i32, name: S) -> Self {
        Self {
            width,
            height,
            rooms: None,
            corridors: None,
            spawn_list: Vec::new(),
            starting_position: None,
            amulet_start: Point::zero(),
            map: Map::new(new_depth, width, height, name),
        }
    }

    pub fn level_builder(
        level: i32,
        width: i32,
        height: i32,
        master: &mut ResMut<TemplateMaster>,
        rng: &mut RandomNumbers,
    ) -> MapBuilder {
        // let mut chain = match level {
        //     _ => random_builder(level, width, height, master),
        //     // 1 => town_builder(new_depth, width, height),
        // };

        let mut chain = random_builder(level, width, height, master, rng);
        chain.map_builder.amulet_start = chain.map_builder.find_most_distant();

        chain.build_map(rng);
        chain.map_builder
    }

    fn find_most_distant(&self) -> Point {
        let start_point = self.starting_position.unwrap_or_else(Point::zero);

        // create the dijstra map from player
        let dijstra_map = DijkstraMap::new(
            self.map.width,
            self.map.height,
            &[self.map.point2d_to_index(start_point)],
            &self.map,
            1024.0,
        );

        const UNREACHABLE: &f32 = &f32::MAX;

        // get the point more far away and return it
        self.map.index_to_point2d(
            dijstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    pub fn spawn_room(&mut self, room: &Rect, master: &mut TemplateMaster, rng: &mut RandomNumbers) {
        let mut possible_targets: Vec<usize> = Vec::new();
        {
            // Borrow scope - to keep access to the map separated
            for y in room.y1 + 1..room.y2 {
                for x in room.x1 + 1..room.x2 {
                    let idx = self.map.xy_idx(x, y);
                    if self.map.tiles[idx] == TileType::Floor {
                        possible_targets.push(idx);
                    }
                }
            }
        }

        self.spawn_region(&possible_targets, master, rng);
    }

    /// Fills a region with stuff!
    pub fn spawn_region(&mut self, area: &[usize], master: &mut TemplateMaster, rng: &mut RandomNumbers) {
        const MAX_MONSTERS: i32 = 10;

        let map_depth = self.map.level;
        let spawn_table = master.get_spawn_table_for_depth(map_depth);
        let mut spawn_points: HashMap<usize, Option<String>> = HashMap::new();
        let mut areas: Vec<usize> = Vec::from(area);

        // Scope to keep the borrow checker happy
        {
            let num_spawns =
                i32::min(areas.len() as i32, rng.roll_dice(1, MAX_MONSTERS + 3) + (map_depth - 1) - 3);
            if num_spawns == 0 {
                return;
            }

            for _i in 0..num_spawns {
                let array_index = if areas.len() == 1 {
                    0usize
                } else {
                    (rng.roll_dice(1, areas.len() as i32) - 1) as usize
                };

                let map_idx = areas[array_index];
                spawn_points.insert(map_idx, spawn_table.roll(rng));
                areas.remove(array_index);
            }
        }

        // Actually spawn the monsters
        for (spawn_idx, spawn_key) in spawn_points.iter() {
            if spawn_key.is_some() {
                self.spawn_list.push((*spawn_idx, spawn_key.as_ref().unwrap().to_string()));
            }
        }
    }
}

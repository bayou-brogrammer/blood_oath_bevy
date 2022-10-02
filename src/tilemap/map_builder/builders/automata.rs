use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect {
    start_floor_percent: i32,
}

impl InitialMapArchitect for CellularAutomataArchitect {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        self.build(builder, rng);
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn build_map(&mut self, builder: &mut MapBuilder, _rng: &RandomNumbers) {
        self.iteration(&mut builder.map);
    }
}

impl CellularAutomataArchitect {
    #[allow(dead_code)]
    pub fn new() -> Box<CellularAutomataArchitect> {
        Box::new(CellularAutomataArchitect { start_floor_percent: 55 })
    }

    pub fn new_with_floor_percent(start_floor_percent: i32) -> Box<CellularAutomataArchitect> {
        Box::new(CellularAutomataArchitect { start_floor_percent })
    }

    fn build(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        self.random_noise_map(&mut builder.map, rng);
        for _ in 0..10 {
            self.iteration(&mut builder.map);
        }
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, map: &mut TileMap, rng: &RandomNumbers) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > self.start_floor_percent {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &TileMap) -> usize {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0)
                    && *map.tiles.get_index_checked(map.xy_idx(x + ix, y + iy))
                        == TileType::Wall
                {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn iteration(&mut self, map: &mut TileMap) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..map.height() - 1 {
            for x in 1..map.width() - 1 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map.xy_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    *new_tiles.get_index_checked_mut(idx) = TileType::Wall;
                } else {
                    *new_tiles.get_index_checked_mut(idx) = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }
}

use super::MapArchitect;
use crate::prelude::*;

pub struct CellularAutomataArchitect {}

impl InitialMapArchitect for CellularAutomataArchitect {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        self.build(builder, rng);
    }
}

impl MapArchitect for CellularAutomataArchitect {
    fn build_map(&mut self, builder: &mut MapBuilder, _rng: &mut RandomNumbers) {
        self.iteration(&mut builder.map);
    }
}

impl CellularAutomataArchitect {
    #[allow(dead_code)]
    pub fn new() -> Box<CellularAutomataArchitect> {
        Box::new(CellularAutomataArchitect {})
    }

    fn build(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        self.random_noise_map(&mut builder.map, rng);
        for _ in 0..10 {
            self.iteration(&mut builder.map);
        }
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(&mut self, map: &mut Map, rng: &mut RandomNumbers) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1..=1 {
            for ix in -1..=1 {
                if !(ix == 0 && iy == 0) && map.tiles[map.xy_idx(x + ix, y + iy)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }

        neighbors
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..map.height - 1 {
            for x in 1..map.width - 1 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map.xy_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }
}

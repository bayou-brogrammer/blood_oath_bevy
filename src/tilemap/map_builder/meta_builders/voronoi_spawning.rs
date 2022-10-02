use crate::prelude::*;
use bracket_noise::prelude::{CellularDistanceFunction, FastNoise, NoiseType};
use std::collections::HashMap;

pub struct VoronoiSpawning {
    spawn_table_name: String,
}

impl MapArchitect for VoronoiSpawning {
    fn build_map(&mut self, build_data: &mut MapBuilder, rng: &RandomNumbers) {
        self.build(build_data, rng);
    }
}

impl VoronoiSpawning {
    pub fn new<S: ToString>(spawn_table_name: S) -> Box<VoronoiSpawning> {
        Box::new(VoronoiSpawning { spawn_table_name: spawn_table_name.to_string() })
    }

    fn build(&mut self, build_data: &mut MapBuilder, rng: &RandomNumbers) {
        let mut noise_areas: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut noise = FastNoise::seeded(rng.roll_dice(1, 65536) as u64);
        noise.set_noise_type(NoiseType::Cellular);
        noise.set_frequency(0.08);
        noise.set_cellular_distance_function(CellularDistanceFunction::Manhattan);

        for y in 1..build_data.map.height() - 1 {
            for x in 1..build_data.map.width() - 1 {
                let idx = build_data.map.xy_idx(x, y);
                if *build_data.map.tiles.get_index_checked_mut(idx) == TileType::Floor {
                    let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.0;
                    let cell_value = cell_value_f as i32;

                    if let std::collections::hash_map::Entry::Vacant(e) =
                        noise_areas.entry(cell_value)
                    {
                        e.insert(vec![idx]);
                    } else {
                        noise_areas.get_mut(&cell_value).unwrap().push(idx);
                    }
                }
            }
        }

        // Spawn the entities
        for area in noise_areas.iter() {
            spawn_region(area.1, &self.spawn_table_name, &mut build_data.spawn_list, rng);
        }
    }
}

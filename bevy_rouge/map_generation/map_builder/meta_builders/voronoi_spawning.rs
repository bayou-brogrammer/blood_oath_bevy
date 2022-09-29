use super::*;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;

pub struct VoronoiSpawning {
    pub master: TemplateMaster,
}

impl MapArchitect for VoronoiSpawning {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        self.build(builder, rng);
    }
}

impl VoronoiSpawning {
    pub fn new(master: &mut TemplateMaster) -> Box<VoronoiSpawning> {
        Box::new(VoronoiSpawning { master: master.clone() })
    }

    fn build(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        let mut noise_areas: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut noise = FastNoise::seeded(rng.roll_dice(1, 65536) as u64);
        noise.set_noise_type(NoiseType::Cellular);
        noise.set_frequency(0.08);
        noise.set_cellular_distance_function(CellularDistanceFunction::Manhattan);

        for y in 1..builder.map.height - 1 {
            for x in 1..builder.map.width - 1 {
                let idx = builder.map.xy_idx(x, y);
                if builder.map.tiles[idx] == TileType::Floor {
                    let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.0;
                    let cell_value = cell_value_f as i32;

                    if let Vacant(e) = noise_areas.entry(cell_value) {
                        e.insert(vec![idx]);
                    } else {
                        noise_areas.get_mut(&cell_value).unwrap().push(idx);
                    }
                }
            }
        }

        // Spawn the entities
        for area in noise_areas.iter() {
            builder.spawn_region(area.1, &mut self.master, rng);
        }
    }
}

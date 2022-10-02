use super::*;

pub struct DistantExit {}

impl MapArchitect for DistantExit {
    fn build_map(&mut self, builder: &mut MapBuilder, _rng: &RandomNumbers) {
        self.build(builder);
    }
}

impl DistantExit {
    pub fn new() -> Box<DistantExit> {
        Box::new(DistantExit {})
    }

    fn build(&mut self, builder: &mut MapBuilder) {
        let starting_pos = *builder.starting_position.as_ref().unwrap();
        let start_idx = builder.map.xy_idx(starting_pos.x, starting_pos.y);
        builder.map.populate_blocked();

        let map_starts: Vec<usize> = vec![start_idx];
        let dijkstra_map = DijkstraMap::new(
            builder.map.width() as usize,
            builder.map.height() as usize,
            &map_starts,
            &builder.map,
            3000.0,
        );

        let mut exit_tile = (0, 0.0f32);
        for (i, tile) in builder.map.tiles.iter_mut().enumerate() {
            if *tile == TileType::Floor {
                let distance_to_start = dijkstra_map.map[i];
                if distance_to_start != std::f32::MAX {
                    // If it is further away than our current exit candidate, move the exit
                    if distance_to_start > exit_tile.1 {
                        exit_tile.0 = i;
                        exit_tile.1 = distance_to_start;
                    }
                }
            }
        }

        // Place a staircase
        builder.place_stairs(exit_tile.0, TileType::DownStairs);
    }
}

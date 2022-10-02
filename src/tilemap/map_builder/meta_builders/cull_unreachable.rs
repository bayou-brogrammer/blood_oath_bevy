use crate::prelude::*;

pub struct CullUnreachable {}

impl MapArchitect for CullUnreachable {
    fn build_map(&mut self, build_data: &mut MapBuilder, _rng: &RandomNumbers) {
        self.build(build_data);
    }
}

impl CullUnreachable {
    pub fn new() -> Box<CullUnreachable> {
        Box::new(CullUnreachable {})
    }

    fn build(&mut self, builder: &mut MapBuilder) {
        let mut seen = Grid::new_copy(builder.map.size, false);
        let player_coord = builder.starting_position.expect("No starting position");
        *seen.get_checked_mut(player_coord) = true;

        let mut to_visit = vec![player_coord];
        while let Some(current) = to_visit.pop() {
            for direction in CardinalDirection::all() {
                let neighbour_coord = current + direction.coord();
                if let Some(neighbour_cell) = builder.map.tiles.get(neighbour_coord) {
                    if !neighbour_cell.is_wall() {
                        let seen_cell = seen.get_checked_mut(neighbour_coord);
                        if !*seen_cell {
                            to_visit.push(neighbour_coord);
                        }
                        *seen_cell = true;
                    }
                }
            }
        }

        for (&seen_cell, map_cell) in seen.iter().zip(builder.map.tiles.iter_mut()) {
            if !seen_cell && map_cell.is_floor() {
                *map_cell = TileType::Wall;
            }
        }
    }
}

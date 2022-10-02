use crate::prelude::*;

pub struct RoomBasedSpawner {
    spawn_table_name: String,
}

impl MapArchitect for RoomBasedSpawner {
    fn build_map(&mut self, build_data: &mut MapBuilder, rng: &RandomNumbers) {
        self.build(build_data, rng);
    }
}

impl RoomBasedSpawner {
    pub fn new<S: ToString>(spawn_table_name: S) -> Box<RoomBasedSpawner> {
        Box::new(RoomBasedSpawner { spawn_table_name: spawn_table_name.to_string() })
    }

    fn build(&mut self, build_data: &mut MapBuilder, rng: &RandomNumbers) {
        if let Some(rooms) = &build_data.rooms {
            for room in rooms.iter().skip(1) {
                spawn_room(
                    &build_data.map,
                    room,
                    &self.spawn_table_name,
                    &mut build_data.spawn_list,
                    rng,
                );
            }
        } else {
            panic!("Room Based Spawning only works after rooms have been created");
        }
    }
}

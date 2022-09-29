use super::*;

pub struct RoomBasedSpawner {
    master: TemplateMaster,
}

impl MapArchitect for RoomBasedSpawner {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        self.build(builder, rng);
    }
}

impl RoomBasedSpawner {
    pub fn new(master: &mut TemplateMaster) -> Box<RoomBasedSpawner> {
        Box::new(RoomBasedSpawner { master: master.clone() })
    }

    fn build(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        if let Some(rooms) = &builder.rooms {
            let mut spawn_rooms = rooms.iter().skip(1).copied().collect::<Vec<_>>();
            spawn_rooms.iter_mut().for_each(|room| builder.spawn_room(room, &mut self.master, rng));
        } else {
            panic!("Room Based Spawning only works after rooms have been created");
        }
    }
}

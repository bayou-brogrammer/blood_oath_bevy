use super::*;

pub struct RoomBasedStartingPosition {}

impl MapArchitect for RoomBasedStartingPosition {
    fn build_map(&mut self, builder: &mut MapBuilder, _rng: &RandomNumbers) {
        self.build(builder);
    }
}

impl RoomBasedStartingPosition {
    pub fn new() -> Box<RoomBasedStartingPosition> {
        Box::new(RoomBasedStartingPosition {})
    }

    fn build(&mut self, builder: &mut MapBuilder) {
        if let Some(rooms) = &builder.rooms {
            let start_pos = rooms[0].center().to_coord();
            builder.starting_position = Some(start_pos);
        } else {
            panic!("Room Based Staring Point only works after rooms have been created");
        }
    }
}

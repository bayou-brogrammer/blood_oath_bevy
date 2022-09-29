use super::*;
pub struct RoomBasedStairs {}

impl MapArchitect for RoomBasedStairs {
    fn build_map(&mut self, builder: &mut MapBuilder, _rng: &mut RandomNumbers) {
        self.build(builder);
    }
}

impl RoomBasedStairs {
    pub fn new() -> Box<RoomBasedStairs> {
        Box::new(RoomBasedStairs {})
    }

    fn build(&mut self, builder: &mut MapBuilder) {
        if let Some(rooms) = &builder.rooms {
            let stairs_position = rooms[rooms.len() - 1].center();
            let stairs_idx = builder.map.point2d_to_index(stairs_position);
            builder.map.tiles[stairs_idx] = TileType::DownStairs;
        } else {
            panic!("Room Based Stairs only works after rooms have been created");
        }
    }
}

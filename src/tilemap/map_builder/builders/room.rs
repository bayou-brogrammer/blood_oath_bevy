use super::*;

pub struct RoomMapArchitect {}

impl InitialMapArchitect for RoomMapArchitect {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        self.build_rooms(builder, rng);
    }
}

impl MapArchitect for RoomMapArchitect {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        self.build_rooms(builder, rng);
    }
}

impl RoomMapArchitect {
    pub fn new() -> Box<RoomMapArchitect> {
        Box::new(RoomMapArchitect {})
    }

    fn build_rooms(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        const MAX_ROOMS: i32 = 25;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rooms: Vec<Rect> = Vec::new();
        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, builder.map.width() - w - 1) - 1;
            let y = rng.roll_dice(1, builder.map.height() - h - 1) - 1;
            let new_room = Rect::with_size(x, y, w, h);

            let ok = rooms.iter().all(|room| !new_room.intersect(room));
            if ok {
                rooms.push(new_room);
            }
        }

        builder.rooms = Some(rooms);
    }
}

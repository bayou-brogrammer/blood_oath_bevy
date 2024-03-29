use super::*;

pub struct BspCorridors {}

impl MapArchitect for BspCorridors {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        self.corridors(builder, rng);
    }
}

impl BspCorridors {
    pub fn new() -> Box<BspCorridors> {
        Box::new(BspCorridors {})
    }

    fn corridors(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        let rooms: Vec<Rect>;
        if let Some(rooms_builder) = &builder.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("BSP Corridors require a builder with room structures");
        }

        let mut corridors: Vec<Vec<usize>> = Vec::new();
        for i in 0..rooms.len() - 1 {
            let room = rooms[i];
            let next_room = rooms[i + 1];
            let start_x = room.x1 + (rng.roll_dice(1, i32::abs(room.x1 - room.x2)) - 1);
            let start_y = room.y1 + (rng.roll_dice(1, i32::abs(room.y1 - room.y2)) - 1);
            let end_x =
                next_room.x1 + (rng.roll_dice(1, i32::abs(next_room.x1 - next_room.x2)) - 1);
            let end_y =
                next_room.y1 + (rng.roll_dice(1, i32::abs(next_room.y1 - next_room.y2)) - 1);
            let corridor = builder.draw_corridor(start_x, start_y, end_x, end_y);
            corridors.push(corridor);
        }

        builder.corridors = Some(corridors);
    }
}

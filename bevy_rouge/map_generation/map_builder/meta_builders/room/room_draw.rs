use super::*;

pub struct RoomDrawer {}

impl MapArchitect for RoomDrawer {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        self.build(builder, rng);
    }
}

impl RoomDrawer {
    pub fn new() -> Box<RoomDrawer> {
        Box::new(RoomDrawer {})
    }

    fn build(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        let rooms: Vec<Rect>;
        if let Some(rooms_builder) = &builder.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Room Drawing require a builder with room structures");
        }

        for room in rooms.iter() {
            let room_type = rng.roll_dice(1, 4);
            match room_type {
                1 => self.circle(builder, room),
                _ => self.rectangle(builder, room),
            }
        }
    }

    fn rectangle(&mut self, builder: &mut MapBuilder, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = builder.map.xy_idx(x, y);
                if idx > 0 && idx < ((builder.map.width * builder.map.height) - 1) as usize {
                    builder.map.tiles[idx] = TileType::Floor;
                }
            }
        }
    }

    fn circle(&mut self, builder: &mut MapBuilder, room: &Rect) {
        let radius = i32::min(room.x2 - room.x1, room.y2 - room.y1) as f32 / 2.0;
        let center = room.center();

        for y in room.y1..=room.y2 {
            for x in room.x1..=room.x2 {
                let idx = builder.map.xy_idx(x, y);
                let distance = DistanceAlg::Pythagoras.distance2d(center, Point::new(x, y));

                if idx > 0
                    && idx < ((builder.map.width * builder.map.height) - 1) as usize
                    && distance <= radius
                {
                    builder.map.tiles[idx] = TileType::Floor;
                }
            }
        }
    }
}

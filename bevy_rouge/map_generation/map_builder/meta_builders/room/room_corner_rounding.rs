use super::*;

pub struct RoomCornerRounder {}

impl MapArchitect for RoomCornerRounder {
    fn build_map(&mut self, builder: &mut MapBuilder, _rng: &mut RandomNumbers) {
        self.build(builder);
    }
}

impl RoomCornerRounder {
    pub fn new() -> Box<RoomCornerRounder> {
        Box::new(RoomCornerRounder {})
    }

    fn fill_if_corner(&mut self, x: i32, y: i32, builder: &mut MapBuilder) {
        let w = builder.map.width;
        let h = builder.map.height;
        let idx = builder.map.xy_idx(x, y);
        let mut neighbor_walls = 0;

        if x > 0 && builder.map.tiles[idx - 1] == TileType::Wall {
            neighbor_walls += 1;
        }
        if y > 0 && builder.map.tiles[idx - w as usize] == TileType::Wall {
            neighbor_walls += 1;
        }
        if x < w - 2 && builder.map.tiles[idx + 1] == TileType::Wall {
            neighbor_walls += 1;
        }
        if y < h - 2 && builder.map.tiles[idx + w as usize] == TileType::Wall {
            neighbor_walls += 1;
        }

        if neighbor_walls == 2 {
            builder.map.tiles[idx] = TileType::Wall;
        }
    }

    fn build(&mut self, builder: &mut MapBuilder) {
        let rooms: Vec<Rect>;
        if let Some(rooms_builder) = &builder.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Room Rounding require a builder with room structures");
        }

        for room in rooms.iter() {
            self.fill_if_corner(room.x1 + 1, room.y1 + 1, builder);
            self.fill_if_corner(room.x2, room.y1 + 1, builder);
            self.fill_if_corner(room.x1 + 1, room.y2, builder);
            self.fill_if_corner(room.x2, room.y2, builder);
        }
    }
}

use super::*;

const MIN_ROOM_SIZE: i32 = 8;

pub struct BspInteriorBuilder {
    rects: Vec<Rect>,
}

impl InitialMapArchitect for BspInteriorBuilder {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        self.build(builder, rng);
    }
}

impl BspInteriorBuilder {
    pub fn new() -> Box<BspInteriorBuilder> {
        Box::new(BspInteriorBuilder { rects: Vec::new() })
    }

    fn build(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        let mut rooms: Vec<Rect> = Vec::new();
        self.rects.clear();

        self.rects.push(Rect::with_size(
            1,
            1,
            builder.map.width() - 1,
            builder.map.height() - 1,
        )); // Start with a single map-sized rectangle
        let first_room = self.rects[0];
        self.add_subrects(first_room, rng); // Divide the first room

        let rooms_copy = self.rects.clone();
        for r in rooms_copy.iter() {
            let room = *r;
            rooms.push(room);
            for y in room.y1..room.y2 {
                for x in room.x1..room.x2 {
                    let idx = builder.map.xy_idx(x, y);
                    if idx > 0
                        && idx < ((builder.map.width() * builder.map.height()) - 1) as usize
                    {
                        *builder.map.tiles.get_index_checked_mut(idx) = TileType::Floor;
                    }
                }
            }
        }

        // Now we want corridors
        for i in 0..rooms.len() - 1 {
            let room = rooms[i];
            let next_room = rooms[i + 1];
            let start_x = room.x1 + (rng.roll_dice(1, i32::abs(room.x1 - room.x2)) - 1);
            let start_y = room.y1 + (rng.roll_dice(1, i32::abs(room.y1 - room.y2)) - 1);
            let end_x =
                next_room.x1 + (rng.roll_dice(1, i32::abs(next_room.x1 - next_room.x2)) - 1);
            let end_y =
                next_room.y1 + (rng.roll_dice(1, i32::abs(next_room.y1 - next_room.y2)) - 1);
            builder.draw_corridor(start_x, start_y, end_x, end_y);
        }

        builder.rooms = Some(rooms);
    }

    fn add_subrects(&mut self, rect: Rect, rng: &RandomNumbers) {
        // Remove the last rect from the list
        if !self.rects.is_empty() {
            self.rects.remove(self.rects.len() - 1);
        }

        // Calculate boundaries
        let width = rect.x2 - rect.x1;
        let height = rect.y2 - rect.y1;
        let half_width = width / 2;
        let half_height = height / 2;

        let split = rng.roll_dice(1, 4);

        if split <= 2 {
            // Horizontal split
            let h1 = Rect::with_size(rect.x1, rect.y1, half_width - 1, height);
            self.rects.push(h1);
            if half_width > MIN_ROOM_SIZE {
                self.add_subrects(h1, rng);
            }

            let h2 = Rect::with_size(rect.x1 + half_width, rect.y1, half_width, height);
            self.rects.push(h2);
            if half_width > MIN_ROOM_SIZE {
                self.add_subrects(h2, rng);
            }
        } else {
            // Vertical split
            let v1 = Rect::with_size(rect.x1, rect.y1, width, half_height - 1);
            self.rects.push(v1);
            if half_height > MIN_ROOM_SIZE {
                self.add_subrects(v1, rng);
            }

            let v2 = Rect::with_size(rect.x1, rect.y1 + half_height, width, half_height);
            self.rects.push(v2);
            if half_height > MIN_ROOM_SIZE {
                self.add_subrects(v2, rng);
            }
        }
    }
}

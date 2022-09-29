use std::cmp::{max, min};

use super::*;

pub struct SimpleMapBuilder {
    map: Map,
    rooms: Vec<Rect>,
    starting_position: Point,
}

impl MapBuilder for SimpleMapBuilder {
    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn starting_pos(&self) -> Point {
        self.starting_position
    }

    fn build_map(&mut self, rng: &RandomNumbers) {
        self.rooms_and_corridors(rng);
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        // for room in self.rooms.iter().skip(1) {
        //     spawner::spawn_room(ecs, room, self.depth);
        // }
    }
}

impl SimpleMapBuilder {
    pub fn new(width: i32, height: i32, new_depth: i32) -> SimpleMapBuilder {
        SimpleMapBuilder {
            rooms: Vec::new(),
            starting_position: Point::zero(),
            map: Map::new(width, height, new_depth, "Room Map"),
        }
    }
    pub fn apply_room_to_map(&mut self, room: &Rect) {
        room.for_each(|pt| {
            let idx = self.map.point2d_to_index(pt);
            self.map.tiles[idx] = TileType::Floor;
        });
    }

    pub fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) -> Vec<usize> {
        let mut corridor = Vec::new();

        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.map.point2d_to_index(Point::new(x, y));
            if self.map.tiles[idx as usize] == TileType::Wall {
                self.map.tiles[idx as usize] = TileType::Floor;
                corridor.push(idx as usize);
            }
        }

        corridor
    }

    pub fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) -> Vec<usize> {
        let mut corridor = Vec::new();

        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.map.point2d_to_index(Point::new(x, y));
            if self.map.tiles[idx as usize] == TileType::Wall {
                self.map.tiles[idx as usize] = TileType::Floor;
                corridor.push(idx as usize);
            }
        }

        corridor
    }

    fn rooms_and_corridors(&mut self, rng: &RandomNumbers) {
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        for _i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, self.map.width - w - 1) - 1;
            let y = rng.roll_dice(1, self.map.height - h - 1) - 1;
            let new_room = Rect::with_size(x, y, w, h);

            let mut ok = true;
            for other_room in self.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }

            if ok {
                self.apply_room_to_map(&new_room);

                if !self.rooms.is_empty() {
                    let Point { x: new_x, y: new_y } = new_room.center();
                    let Point { x: prev_x, y: prev_y } = self.rooms[self.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        self.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        self.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        self.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        self.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                self.rooms.push(new_room);
            }
        }

        let stairs_position = self.rooms[self.rooms.len() - 1].center();
        let stairs_idx = self.map.point2d_to_index(stairs_position);
        self.map.tiles[stairs_idx] = TileType::DownStairs;

        self.starting_position = self.rooms[0].center();
    }
}

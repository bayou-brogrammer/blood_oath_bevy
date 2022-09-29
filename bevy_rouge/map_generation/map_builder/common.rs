use super::*;
use std::cmp::{max, min};

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Symmetry {
    None,
    Both,
    Vertical,
    Horizontal,
}

impl MapBuilder {
    pub fn apply_room_to_map(&self, map: &mut Map, room: &Rect) {
        room.for_each(|pt| {
            let idx = map.point2d_to_index(pt);
            map.tiles[idx] = TileType::Floor;
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

    pub fn draw_corridor(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<usize> {
        let mut corridor = Vec::new();
        let mut x = x1;
        let mut y = y1;

        while x != x2 || y != y2 {
            if x < x2 {
                x += 1;
            } else if x > x2 {
                x -= 1;
            } else if y < y2 {
                y += 1;
            } else if y > y2 {
                y -= 1;
            }

            let idx = self.map.xy_idx(x, y);
            if self.map.tiles[idx] != TileType::Floor {
                corridor.push(idx);
                self.map.tiles[idx] = TileType::Floor;
            }
        }

        corridor
    }

    pub fn place_stairs(&mut self, stairs_idx: usize, stair_type: TileType) {
        // Place a staircase
        self.map.tiles[stairs_idx] = match stair_type {
            TileType::DownStairs => TileType::DownStairs,
            TileType::UpStairs => TileType::UpStairs,
            _ => panic!("Invalid stair type"),
        };
    }

    pub fn paint(&mut self, mode: Symmetry, brush_size: i32, x: i32, y: i32) {
        match mode {
            Symmetry::None => self.apply_paint(brush_size, x, y),
            Symmetry::Horizontal => {
                let center_x = self.map.width / 2;
                if x == center_x {
                    self.apply_paint(brush_size, x, y);
                } else {
                    let dist_x = i32::abs(center_x - x);
                    self.apply_paint(brush_size, center_x + dist_x, y);
                    self.apply_paint(brush_size, center_x - dist_x, y);
                }
            }
            Symmetry::Vertical => {
                let center_y = self.map.height / 2;
                if y == center_y {
                    self.apply_paint(brush_size, x, y);
                } else {
                    let dist_y = i32::abs(center_y - y);
                    self.apply_paint(brush_size, x, center_y + dist_y);
                    self.apply_paint(brush_size, x, center_y - dist_y);
                }
            }
            Symmetry::Both => {
                let center_x = self.map.width / 2;
                let center_y = self.map.height / 2;
                if x == center_x && y == center_y {
                    self.apply_paint(brush_size, x, y);
                } else {
                    let dist_x = i32::abs(center_x - x);
                    self.apply_paint(brush_size, center_x + dist_x, y);
                    self.apply_paint(brush_size, center_x - dist_x, y);
                    let dist_y = i32::abs(center_y - y);
                    self.apply_paint(brush_size, x, center_y + dist_y);
                    self.apply_paint(brush_size, x, center_y - dist_y);
                }
            }
        }
    }

    fn apply_paint(&mut self, brush_size: i32, x: i32, y: i32) {
        match brush_size {
            1 => {
                let digger_idx = self.map.xy_idx(x, y);
                self.map.tiles[digger_idx] = TileType::Floor;
            }

            _ => {
                let half_brush_size = brush_size / 2;
                for brush_y in y - half_brush_size..y + half_brush_size {
                    for brush_x in x - half_brush_size..x + half_brush_size {
                        if brush_x > 1
                            && brush_x < self.map.width - 1
                            && brush_y > 1
                            && brush_y < self.map.height - 1
                        {
                            let idx = self.map.xy_idx(brush_x, brush_y);
                            self.map.tiles[idx] = TileType::Floor;
                        }
                    }
                }
            }
        }
    }
}

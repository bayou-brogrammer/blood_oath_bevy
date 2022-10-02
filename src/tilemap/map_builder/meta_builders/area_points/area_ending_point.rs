#![allow(dead_code)]

use super::*;

pub enum XEnd {
    Left,
    Center,
    Right,
}

pub enum YEnd {
    Top,
    Center,
    Bottom,
}

pub struct AreaEndingPosition {
    x: XEnd,
    y: YEnd,
}

impl MapArchitect for AreaEndingPosition {
    fn build_map(&mut self, builder: &mut MapBuilder, _rng: &RandomNumbers) {
        self.build(builder);
    }
}

impl AreaEndingPosition {
    pub fn new(x: XEnd, y: YEnd) -> Box<AreaEndingPosition> {
        Box::new(AreaEndingPosition { x, y })
    }

    fn build(&mut self, builder: &mut MapBuilder) {
        let seed_x = match self.x {
            XEnd::Left => 1,
            XEnd::Center => builder.map.width() / 2,
            XEnd::Right => builder.map.width() - 2,
        };

        let seed_y = match self.y {
            YEnd::Top => 1,
            YEnd::Center => builder.map.height() / 2,
            YEnd::Bottom => builder.map.height() - 2,
        };

        let mut available_floors: Vec<(usize, f32)> = Vec::new();
        for (idx, tile) in builder.map.tiles.iter().enumerate() {
            if tile.is_walkable() {
                let pt = builder.map.index_to_point2d(idx);

                available_floors.push((
                    idx,
                    DistanceAlg::PythagorasSquared.distance2d(pt, Point::new(seed_x, seed_y)),
                ));
            }
        }
        if available_floors.is_empty() {
            panic!("No valid floors to start on");
        }

        available_floors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        *builder.map.tiles.get_index_checked_mut(available_floors[0].0) = TileType::DownStairs
    }
}

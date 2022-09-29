use super::*;

pub enum XStart {
    Left,
    Center,
    Right,
}

pub enum YStart {
    Top,
    Center,
    Bottom,
}

pub struct AreaStartingPosition {
    x: XStart,
    y: YStart,
}

impl MapArchitect for AreaStartingPosition {
    fn build_map(&mut self, builder: &mut MapBuilder, _rng: &mut RandomNumbers) {
        self.build(builder);
    }
}

impl AreaStartingPosition {
    pub fn new(x: XStart, y: YStart) -> Box<AreaStartingPosition> {
        Box::new(AreaStartingPosition { x, y })
    }

    fn build(&mut self, builder: &mut MapBuilder) {
        let seed_x = match self.x {
            XStart::Left => 1,
            XStart::Center => builder.map.width / 2,
            XStart::Right => builder.map.width - 2,
        };

        let seed_y = match self.y {
            YStart::Top => 1,
            YStart::Center => builder.map.height / 2,
            YStart::Bottom => builder.map.height - 2,
        };

        let mut available_floors: Vec<(usize, f32)> = Vec::new();
        for (idx, tile) in builder.map.tiles.iter().enumerate() {
            if tile.walkable() {
                let pt = builder.map.index_to_point2d(idx);
                available_floors
                    .push((idx, DistanceAlg::PythagorasSquared.distance2d(pt, Point::new(seed_x, seed_y))));
            }
        }
        if available_floors.is_empty() {
            panic!("No valid floors to start on");
        }

        available_floors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let pt = builder.map.index_to_point2d(available_floors[0].0);
        builder.starting_position = Some(pt);
    }
}

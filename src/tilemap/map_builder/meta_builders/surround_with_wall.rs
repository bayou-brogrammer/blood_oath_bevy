use super::*;

pub struct SurroundWithWall {}

impl MapArchitect for SurroundWithWall {
    fn build_map(&mut self, builder: &mut MapBuilder, _rng: &RandomNumbers) {
        self.build(builder);
    }
}

impl SurroundWithWall {
    pub fn new() -> Box<SurroundWithWall> {
        Box::new(SurroundWithWall {})
    }

    fn build(&mut self, builder: &mut MapBuilder) {
        for (idx, tile) in builder.map.tiles.rows_mut().enumerate() {
            if idx == 0 || idx as u32 == builder.map.size.height() - 1 {
                for tile in tile {
                    *tile = TileType::Wall;
                }
            } else {
                *tile.first_mut().unwrap() = TileType::Wall;
                *tile.last_mut().unwrap() = TileType::Wall;
            }
        }
    }
}

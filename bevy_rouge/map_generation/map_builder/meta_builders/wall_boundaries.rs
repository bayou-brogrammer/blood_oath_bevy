use super::*;

pub struct WallBoundaries {}

impl MapArchitect for WallBoundaries {
    fn build_map(&mut self, builder: &mut MapBuilder, _rng: &mut RandomNumbers) {
        self.build(builder);
    }
}

impl WallBoundaries {
    pub fn new() -> Box<WallBoundaries> {
        Box::new(WallBoundaries {})
    }

    fn build(&mut self, builder: &mut MapBuilder) {
        // Make the boundaries walls
        for x in 0..builder.map.width {
            let idx_1 = builder.map.xy_idx(x, 1);
            let idx_2 = builder.map.xy_idx(x, builder.map.height - 1);

            builder.map.tiles[idx_1] = TileType::Wall;
            builder.map.tiles[idx_2] = TileType::Wall;
        }

        for y in 0..builder.map.height {
            let idx_1 = builder.map.xy_idx(1, y);
            let idx_2 = builder.map.xy_idx(builder.map.width - 1, y);

            builder.map.tiles[idx_1] = TileType::Wall;
            builder.map.tiles[idx_2] = TileType::Wall;
        }
    }
}

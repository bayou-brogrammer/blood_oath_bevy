use crate::prelude::*;

impl TileMap {
    #[allow(clippy::match_single_binding)]
    pub fn tile_glyph(&self, coord: Coord) -> (FontCharType, ColorPair) {
        let idx = self.coord_to_index(coord);
        let tile = &self.tiles.get_index_checked(idx);
        let (glyph, mut color) = match self.depth {
            _ => tile.get_tile_glyph_default(self, idx),
        };

        if !self.is_visible(self.index_to_coord(idx)) {
            color.fg = RGBA::from(RGB::new_grey(33));
            if color.bg != RGBA::named(BLACK) {
                color.bg = RGBA::from(RGB::new_grey(33));
            }
        }

        (glyph, color)
    }
}

impl TileType {
    pub fn get_tile_glyph_default(
        &self,
        map: &TileMap,
        idx: usize,
    ) -> (FontCharType, ColorPair) {
        let (glyph, fg, bg) = match self {
            TileType::Wall => {
                let is_wall_below = if let Some(tile) = map.tiles.get(map.index_to_coord(idx))
                {
                    tile.is_wall()
                } else {
                    false
                };

                if is_wall_below {
                    (to_cp437(' '), RGB::from_u8(68, 39, 14), RGB::from_u8(125, 82, 44))
                } else {
                    (to_cp437('▄'), RGB::from_u8(68, 39, 14), RGB::from_u8(125, 82, 44))
                }
            }
            TileType::DownStairs => (to_cp437('>'), RGB::named(CYAN), RGB::named(BLACK)),
            TileType::UpStairs => (to_cp437('<'), RGB::named(CYAN), RGB::named(BLACK)),
            TileType::Floor => (to_cp437('.'), RGB::new_grey(127), RGB::named(BLACK)),
            TileType::Door => (to_cp437('+'), RGB::named(CHOCOLATE), RGB::named(BLACK)),
        };

        (glyph, ColorPair::new(fg, bg))
    }
}

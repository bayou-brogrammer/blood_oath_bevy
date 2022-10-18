use crate::prelude::*;

pub const AMBIENT_COL: u8 = 75;

impl TileMap {
    #[allow(clippy::match_single_binding)]
    pub fn tile_glyph(&self, coord: Coord) -> (FontCharType, ColorPair) {
        let idx = self.coord_to_index(coord);
        let tile = &self.tiles.get_index_checked(idx);
        let (glyph, mut color) = match self.depth {
            _ => tile.get_tile_glyph_default(self, idx),
        };

        if !self.is_visible(self.index_to_coord(idx)) {
            color.fg = color.fg.saturating_scalar_mul_div(1, 3);
            if color.bg != RGBA::named(BLACK) {
                color.bg = color.bg.saturating_scalar_mul_div(1, 3);
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
                let coord = map.index_to_coord(idx);
                (wall_glyph(map, coord), RGB::new_grey(127), RGB::named(BLACK))
            }
            TileType::DownStairs => (to_cp437('>'), RGB::named(CYAN), RGB::named(BLACK)),
            TileType::UpStairs => (to_cp437('<'), RGB::named(CYAN), RGB::named(BLACK)),
            TileType::Floor => (to_cp437('.'), RGB::named(WHITE), RGB::named(BLACK)),
            TileType::Door => (to_cp437('+'), RGB::named(CHOCOLATE), RGB::named(BLACK)),
        };

        (glyph, ColorPair::new(fg, bg))
    }
}

fn wall_glyph(map: &TileMap, coord: Coord) -> FontCharType {
    if coord.x < 1 || coord.x > map.width() - 1 || coord.y < 1 || coord.y > map.height() - 1 {
        return to_cp437('#');
    }

    let mut mask: u8 = 0;
    if is_revealed_and_wall(map, coord + Coord::new(0, 1)) {
        mask += 1;
    }

    match mask {
        0 => to_cp437('#'),
        1 => to_cp437('#'),
        _ => to_cp437('#'),
    }
}

fn is_revealed_and_wall(map: &TileMap, coord: Coord) -> bool {
    if map.in_bounds(coord) {
        let idx = map.coord_to_index(coord);
        *map.tiles.get_index_checked(idx) == TileType::Wall && map.revealed.get_bit(idx)
    } else {
        false
    }
}

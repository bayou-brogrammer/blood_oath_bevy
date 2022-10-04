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

                // let pt = map.index_to_point2d(idx);
                // (wall_glyph(map, pt.x, pt.y), RGB::from_u8(8, 39, 14), RGB::named(BLACK))
            }
            TileType::DownStairs => (to_cp437('>'), RGB::named(CYAN), RGB::named(BLACK)),
            TileType::UpStairs => (to_cp437('<'), RGB::named(CYAN), RGB::named(BLACK)),
            TileType::Floor => (to_cp437('.'), RGB::new_grey(127), RGB::named(BLACK)),
            TileType::Door => (to_cp437('+'), RGB::named(CHOCOLATE), RGB::named(BLACK)),
        };

        (glyph, ColorPair::new(fg, bg))
    }
}

#[rustfmt::skip]
fn wall_glyph(map: &TileMap, x: i32, y: i32) -> FontCharType {
    if x < 1 || x > map.width() - 1 || y < 1 || y > map.height() - 1 {
        return 35;
    }
    let mut mask : u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) { mask +=1; }
    if is_revealed_and_wall(map, x, y + 1) { mask +=2; }
    if is_revealed_and_wall(map, x - 1, y) { mask +=4; }
    if is_revealed_and_wall(map, x + 1, y) { mask +=8; }

    match mask {
        0 => { 9 } // Pillar because we can't see neighbors
        1 => { 186 } // Wall only to the north
        2 => { 186 } // Wall only to the south
        3 => { 186 } // Wall to the north and south
        4 => { 205 } // Wall only to the west
        5 => { 188 } // Wall to the north and west
        6 => { 187 } // Wall to the south and west
        7 => { 185 } // Wall to the north, south and west
        8 => { 205 } // Wall only to the east
        9 => { 200 } // Wall to the north and east
        10 => { 201 } // Wall to the south and east
        11 => { 204 } // Wall to the north, south and east
        12 => { 205 } // Wall to the east and west
        13 => { 202 } // Wall to the east, west, and south
        14 => { 203 } // Wall to the east, west, and north
        15 => { 206 }  // ╬ Wall on all sides
        _ => { 35 } // We missed one?
    }
}

fn is_revealed_and_wall(map: &TileMap, x: i32, y: i32) -> bool {
    let pt = Coord::new(x, y);
    if map.in_bounds(pt) {
        let idx = map.coord_to_index(pt);
        *map.tiles.get_index_checked(idx) == TileType::Wall && map.revealed.get_bit(idx)
    } else {
        false
    }
}

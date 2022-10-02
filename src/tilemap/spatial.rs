use super::*;

/// Visibility Functionality
impl TileMap {
    pub fn clear_visible(&mut self) {
        self.visible.zero_out_bits();
    }

    pub fn set_revealed_and_visible(&mut self, coord: Coord) {
        if self.in_bounds(coord) {
            let idx = self.coord_to_index(coord);
            self.visible.set_bit(idx, true);
            self.revealed.set_bit(idx, true);
        }
    }

    pub fn is_visible(&self, coord: Coord) -> bool {
        self.visible.get_bit(self.coord_to_index(coord))
    }

    pub fn is_revealed(&self, coord: Coord) -> bool {
        self.revealed.get_bit(self.coord_to_index(coord))
    }
}

/// Blocked Functionality
impl TileMap {
    pub fn populate_blocked(&mut self) {
        self.tiles
            .iter()
            .enumerate()
            .for_each(|(idx, b)| self.blocked.set_bit(idx, !b.is_walkable()));
    }

    pub fn is_blocked(&self, idx: usize) -> bool {
        self.blocked.get_bit(idx)
    }

    pub fn clear_blocked(&mut self) {
        self.blocked.zero_out_bits();
    }
}

/// Opaque Functionality
impl TileMap {
    pub fn populate_opaque(&mut self) {
        self.tiles
            .iter()
            .enumerate()
            .for_each(|(idx, b)| self.opaque.set_bit(idx, b.is_opaque()));
    }

    pub fn is_opaque(&self, idx: usize) -> bool {
        self.opaque.get_bit(idx)
    }

    pub fn clear_opaque(&mut self) {
        self.opaque.zero_out_bits();
    }
}

/// Tile Content Functionality
impl TileMap {
    pub fn get_tile_content_pt(&self, pt: Coord) -> impl Iterator<Item = Entity> + '_ {
        let idx = self.coord_to_index(pt);
        self.tile_content[idx].iter().map(|(e, _, _)| *e)
    }

    pub fn get_tile_content_pt_clone(&self, pt: Coord) -> Vec<Entity> {
        let idx = self.coord_to_index(pt);
        self.tile_content[idx].iter().map(|(e, _, _)| *e).collect::<Vec<_>>()
    }
}

/// Indexing Functionality
impl TileMap {
    pub fn index_entity(
        &mut self,
        entity: Entity,
        idx: usize,
        blocks_tile: bool,
        blocks_visibility: bool,
    ) {
        self.tile_content[idx].push((entity, blocks_tile, blocks_visibility));

        if blocks_tile {
            self.blocked.set_bit(idx, true);
        }
        if blocks_visibility {
            self.opaque.set_bit(idx, true);
        }
    }

    pub fn move_entity(&mut self, entity: Entity, moving_from: Coord, moving_to: Coord) {
        let from_idx = self.coord_to_index(moving_from);
        let to_idx = self.coord_to_index(moving_to);

        let mut entity_blocks = false;
        let mut entity_opaque = false;
        self.tile_content[from_idx].retain(|(e, blocks, opaque)| {
            if *e == entity {
                entity_blocks = *blocks;
                entity_opaque = *opaque;
                false
            } else {
                true
            }
        });
        self.tile_content[to_idx].push((entity, entity_blocks, entity_opaque));

        // Recalculate blocks for both tiles
        let mut from_blocked = false;
        let mut to_blocked = false;
        let mut from_opaque = false;
        let mut to_opaque = false;
        self.tile_content[from_idx].iter().for_each(|(_, blocks, opaque)| {
            if *blocks {
                from_blocked = true;
            }
            if *opaque {
                from_opaque = true;
            }
        });
        self.tile_content[to_idx].iter().for_each(|(_, blocks, opaque)| {
            if *blocks {
                to_blocked = true;
            }
            if *opaque {
                to_opaque = true;
            }
        });

        self.blocked.set_bit(from_idx, from_blocked);
        self.blocked.set_bit(to_idx, to_blocked);

        self.opaque.set_bit(from_idx, from_opaque);
        self.opaque.set_bit(to_idx, to_opaque);
    }

    pub fn remove_entity(&mut self, entity: Entity, pt: Coord) {
        let idx = self.coord_to_index(pt);
        self.tile_content[idx].retain(|(e, _, _)| *e != entity);

        let mut from_blocked = false;
        let mut from_opaque = false;
        self.tile_content[idx].iter().for_each(|(_, blocks, opaque)| {
            if *blocks {
                from_blocked = true;
            }
            if *opaque {
                from_opaque = true;
            }
        });

        self.blocked.set_bit(idx, from_blocked);
        self.opaque.set_bit(idx, from_opaque);
    }
}

// pub fn for_each_tile_content<F>(idx: usize, mut f: F)
// where
//     F: FnMut(Entity),
// {
//     let lock = SPATIAL_MAP.lock();
//     for entity in lock.tile_content[idx].iter() {
//         f(entity.0);
//     }
// }

// pub fn for_each_tile_content_pt<F>(pt: Coord, mut f: F)
// where
//     F: FnMut(Entity),
// {
//     let lock = SPATIAL_MAP.lock();
//     let idx = lock.coord_to_index(pt);
//     for entity in lock.tile_content[idx].iter() {
//         f(entity.0);
//     }
// }

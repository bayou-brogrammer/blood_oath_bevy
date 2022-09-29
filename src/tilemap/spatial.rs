use super::*;

/// Blocked Functionality
impl TileMap {
    pub fn clear(&mut self) {
        self.clear_blocked();
        self.clear_opaque();
        self.tile_content.iter_mut().for_each(|v| v.clear());
    }

    pub fn populate_blocked(&mut self) {
        self.tiles
            .iter_mut()
            .for_each(|tile| tile.flags.set(TileFlags::BLOCKS_MOVEMENT, !tile.walkable()));
    }

    pub fn is_blocked(&self, idx: usize) -> bool {
        self.tiles[idx].flags.contains(TileFlags::BLOCKS_MOVEMENT)
    }

    pub fn clear_blocked(&mut self) {
        self.tiles.iter_mut().for_each(|tile| tile.flags.remove(TileFlags::BLOCKS_MOVEMENT));
    }
}

/// Opaque Functionality
impl TileMap {
    pub fn populate_opaque(&mut self) {
        self.tiles
            .iter_mut()
            .for_each(|tile| tile.flags.set(TileFlags::BLOCKS_VISION, tile.is_opaque()))
    }

    pub fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx].flags.contains(TileFlags::BLOCKS_VISION)
    }

    pub fn clear_opaque(&mut self) {
        self.tiles.iter_mut().for_each(|tile| tile.flags.remove(TileFlags::BLOCKS_VISION));
    }
}

/// Tile Content Functionality
impl TileMap {
    pub fn get_tile_content_pt(&self, pt: Point) -> impl Iterator<Item = Entity> + '_ {
        let idx = self.point2d_to_index(pt);
        self.tile_content[idx].iter().map(|(e, _, _)| *e)
    }

    pub fn get_tile_content_pt_clone(&self, pt: Point) -> Vec<Entity> {
        let idx = self.point2d_to_index(pt);
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
            self.tiles[idx].flags.insert(TileFlags::BLOCKS_MOVEMENT);
        }
        if blocks_visibility {
            self.tiles[idx].flags.insert(TileFlags::BLOCKS_VISION);
        }
    }

    pub fn move_entity(&mut self, entity: Entity, moving_from: Point, moving_to: Point) {
        let from_idx = self.point2d_to_index(moving_from);
        let to_idx = self.point2d_to_index(moving_to);

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

        self.tiles[from_idx].flags.set(TileFlags::BLOCKS_MOVEMENT, from_blocked);
        self.tiles[to_idx].flags.set(TileFlags::BLOCKS_MOVEMENT, to_blocked);

        self.tiles[from_idx].flags.set(TileFlags::BLOCKS_VISION, from_opaque);
        self.tiles[to_idx].flags.set(TileFlags::BLOCKS_VISION, to_opaque);
    }

    pub fn remove_entity(&mut self, entity: Entity, pt: Point) {
        let idx = self.point2d_to_index(pt);
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

        self.tiles[idx].flags.set(TileFlags::BLOCKS_MOVEMENT, from_blocked);
        self.tiles[idx].flags.set(TileFlags::BLOCKS_VISION, from_opaque);
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

// pub fn for_each_tile_content_pt<F>(pt: Point, mut f: F)
// where
//     F: FnMut(Entity),
// {
//     let lock = SPATIAL_MAP.lock();
//     let idx = lock.point2d_to_index(pt);
//     for entity in lock.tile_content[idx].iter() {
//         f(entity.0);
//     }
// }

use super::*;

impl Map {
    pub fn clear(&mut self) {
        self.blocked.iter_mut().for_each(|b| {
            b.0 = false;
            b.1 = false;
        });

        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter().enumerate() {
            self.blocked[i].0 = !tile.walkable();
        }
    }

    pub fn index_entity(&mut self, entity: Entity, idx: usize, blocks_tile: bool) {
        self.tile_content[idx].push((entity, blocks_tile));
        if blocks_tile {
            self.blocked[idx].1 = true;
        }
    }

    pub fn is_blocked(&self, idx: usize) -> bool { self.blocked[idx].0 || self.blocked[idx].1 }

    pub fn set_blocked(&mut self, idx: usize, blocked: bool) {
        self.blocked[idx] = (self.blocked[idx].0, blocked);
    }

    pub fn move_entity(&mut self, entity: Entity, moving_from: Point, moving_to: Point) {
        let moving_from_idx = self.point2d_to_index(moving_from);
        let moving_to_idx = self.point2d_to_index(moving_to);

        let mut entity_blocks = false;
        self.tile_content[moving_from_idx].retain(|(e, blocks)| {
            if *e == entity {
                entity_blocks = *blocks;
                false
            } else {
                true
            }
        });
        self.tile_content[moving_to_idx].push((entity, entity_blocks));

        // Recalculate blocks for both tiles
        let mut from_blocked = false;
        let mut to_blocked = false;
        self.tile_content[moving_from_idx].iter().for_each(|(_, blocks)| {
            if *blocks {
                from_blocked = true;
            }
        });

        self.tile_content[moving_to_idx].iter().for_each(|(_, blocks)| {
            if *blocks {
                to_blocked = true;
            }
        });
        self.blocked[moving_from_idx].1 = from_blocked;
        self.blocked[moving_to_idx].1 = to_blocked;
    }

    pub fn remove_entity(&mut self, entity: Entity, pt: Point) {
        let idx = self.point2d_to_index(pt);
        self.tile_content[idx].retain(|(e, _)| *e != entity);

        let mut from_blocked = false;
        self.tile_content[idx].iter().for_each(|(_, blocks)| {
            if *blocks {
                from_blocked = true;
            }
        });
        self.blocked[idx].1 = from_blocked;
    }
}

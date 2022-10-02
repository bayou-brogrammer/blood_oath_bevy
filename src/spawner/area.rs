use std::collections::HashMap;

use crate::prelude::*;

const MAX_MONSTERS: i32 = 2;

fn room_table(table_name: &str) -> MasterTable {
    get_spawn_table_by_name(table_name)
}

/// Fills a room with stuff!
pub fn spawn_room(
    map: &TileMap,
    room: &Rect,
    table_name: &str,
    spawn_list: &mut Vec<(usize, String)>,
    rng: &RandomNumbers,
) {
    let mut possible_targets: Vec<usize> = Vec::new();
    {
        // Borrow scope - to keep access to the map separated
        for y in room.y1 + 1..room.y2 {
            for x in room.x1 + 1..room.x2 {
                let idx = map.xy_idx(x, y);
                if map.tiles.get_index_checked(idx).is_walkable() {
                    possible_targets.push(idx);
                }
            }
        }
    }

    spawn_region(&possible_targets, table_name, spawn_list, rng);
}

/// Fills a region with stuff!
pub fn spawn_region(
    area: &[usize],
    table_name: &str,
    spawn_list: &mut Vec<(usize, String)>,
    rng: &RandomNumbers,
) {
    let spawn_table = room_table(table_name);
    let mut spawn_points: HashMap<usize, Option<String>> = HashMap::new();
    let mut areas: Vec<usize> = Vec::from(area);

    // Scope to keep the borrow checker happy
    {
        let num_spawns = i32::min(areas.len() as i32, rng.roll_dice(1, MAX_MONSTERS));
        if num_spawns == 0 {
            return;
        }

        for _i in 0..num_spawns {
            let array_index = if areas.len() == 1 {
                0usize
            } else {
                (rng.roll_dice(1, areas.len() as i32) - 1) as usize
            };

            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, spawn_table.roll(rng));
            areas.remove(array_index);
        }
    }

    // Actually spawn the monsters
    for (spawn_idx, spawn_key) in spawn_points.iter() {
        if spawn_key.is_some() {
            spawn_list.push((*spawn_idx, spawn_key.as_ref().unwrap().to_string()));
        }
    }
}

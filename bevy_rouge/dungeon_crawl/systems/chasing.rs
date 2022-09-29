use crate::dungeon_crawl::*;

pub fn chasing(
    mut commands: Commands,
    mb: Res<MapBuilder>,
    movers: Query<(Entity, &Point, &FieldOfView), With<ChasingPlayer>>,
    positions: Query<(Entity, &Point), With<Health>>,
    player: Query<(Entity, &Point), With<Player>>,
) {
    let (_, player_pos) = player.single();

    // just get the map
    let map = &mb.map;

    // transform x,y position to index in array
    let player_idx = map.point2d_to_index(*player_pos);

    // create dijkstra map around player
    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(map.width, map.height, &search_targets, map, 1024.0);

    movers.iter().for_each(|(entity, pos, fov)| {
        // if monster cannot see player, then just return and do nothing
        if !fov.visible_tiles.contains(player_pos) {
            return;
        }

        let idx = map.point2d_to_index(*pos);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, &mb.map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            let destination: Point =
                if distance > 1.2 { map.index_to_point2d(destination) } else { *player_pos };

            let mut attacked = false;
            positions.iter().filter(|(_, target_pos)| **target_pos == destination).for_each(|(victim, _)| {
                // if the victim is the player
                if let Ok((player_victim, _)) = player.get(victim) {
                    // send an attack message
                    commands.spawn().insert(WantsToAttack { attacker: entity, victim: player_victim });
                }
                attacked = true;
            });

            if !attacked {
                // move to new position
                commands.spawn().insert(WantsToMove { entity, destination });
            }
        }
    });
}

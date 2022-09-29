use crate::prelude::*;

pub fn chasing(
    map: Res<Map>,
    player: Query<&Point, With<Player>>,
    mut move_events: EventWriter<WantsToMove>,
    movers: Query<(Entity, &Point, &FieldOfView), (With<ChasingPlayer>, With<MyTurn>)>,
) {
    let player_pos = player.single();
    let player_idx = map.point2d_to_index(*player_pos);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map.as_ref(), 1024.0);

    for (entity, pos, fov) in movers.iter() {
        // if !fov.visible_tiles.contains(player_pos) {
        //     continue;
        // }

        let idx = map.point2d_to_index(*pos);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map.as_ref()) {
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);
            let destination = if distance > 1.2 { map.index_to_point2d(destination) } else { *player_pos };

            println!("AI");
            move_events.send(WantsToMove(entity, *pos, destination));
        }
    }
}

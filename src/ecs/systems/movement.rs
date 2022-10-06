use crate::prelude::*;

pub fn movement(
    mut map: ResMut<TileMap>,
    mut move_events: ResMut<Events<WantsToMove>>,
    mut pos_q: Query<(&mut Position, Option<&Player>)>,
    mut cam: ResMut<CameraView>,
    // turn_q: Query<&MyTurn>,
    // stats_q: Query<&Stats>,
    // mut commands: Commands,
) {
    for WantsToMove(entity, destination) in move_events.drain() {
        if map.can_enter_tile(destination) {
            if let Ok((mut pos, player)) = pos_q.get_mut(entity) {
                pos.0 = destination;
                map.move_entity(entity, pos.0, destination);

                if player.is_some() {
                    cam.on_player_move(destination);
                }
            }

            // if let Ok(stats) = stats_q.get(entity) {
            //     end_turn_requeue(&mut commands, entity, &turn_q, stats.movement_cost);
            // } else {
            //     panic!("Something moved that doesn't have Stats")
            // }
        }
    }
}

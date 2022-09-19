use crate::prelude::*;

pub fn movement(
    map: Res<TileMap>,
    mut move_events: ResMut<Events<WantsToMove>>,
    mut pos_q: Query<&mut Position>,
    // turn_q: Query<&MyTurn>,
    // mut commands: Commands,
) {
    for WantsToMove(entity, destination) in move_events.drain() {
        if map.in_bounds(destination) {
            if let Ok(mut pos) = pos_q.get_mut(entity) {
                // let start_idx = map.point2d_to_index(pos.0);
                // let dest_idx = map.point2d_to_index(destination);

                // crate::spatial::move_entity(entity, start_idx, dest_idx);

                pos.0 = destination;
            }

            // if let Ok(stats) = stats_q.get(entity) {
            //     end_turn_requeue(&mut commands, entity, &turn_q, stats.movement_cost);
            // } else {
            //     panic!("Something moved that doesn't have Stats")
            // }
        }
    }
}

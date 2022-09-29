use crate::dungeon_crawl::*;

pub fn movement(
    mut map: ResMut<Map>,
    mut commands: Commands,
    move_messages: Query<(Entity, &WantsToMove)>,
    mut movers: Query<(Entity, &Point, &mut FieldOfView)>,
) {
    // for every message to move
    for (message_ent, move_signal) in move_messages.iter() {
        // if the movement is physically valid
        if map.can_enter_tile(move_signal.destination) {
            // if no other character is in that cell
            if let Ok((mov_ent, position, mut fov)) = movers.get_mut(move_signal.entity) {
                // update occupation map
                map.move_entity(mov_ent, *position, move_signal.destination);
                commands.entity(mov_ent).insert(move_signal.destination);

                // mark the fov to be updated
                fov.is_dirty = true;
            }
        }
        // delete the message
        commands.entity(message_ent).despawn();
    }
}

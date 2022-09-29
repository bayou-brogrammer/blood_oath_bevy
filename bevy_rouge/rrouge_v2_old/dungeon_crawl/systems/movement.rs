use crate::dungeon_crawl::*;

pub fn movement(
    mut map: ResMut<Map>,
    mut commands: Commands,
    // move_messages: Query<(Entity, &WantsToMove)>,
    // mut movers: Query<(Entity, &Point, &mut FieldOfView)>,
    mut events: EventReader<WantsToMove>,
) {
    // for every message to move
    for WantsToMove(entity, from, destination) in events.iter() {
        // if the movement is physically valid
        if map.can_enter_tile(*destination) {
            // update occupation map
            map.move_entity(*entity, *from, *destination);
            commands.entity(*entity).insert(*destination);

            // if no other character is in that cell
            // if let Ok((mov_ent, position, mut fov)) = movers.get_mut(move_signal.entity) {
            //     // update occupation map
            //     map.move_entity(mov_ent, *position, move_signal.destination);
            //     commands.entity(mov_ent).insert(move_signal.destination);

            //     // mark the fov to be updated
            //     fov.is_dirty = true;
            // }
        }

        // delete the message
        // commands.entity(message_ent).despawn();
    }
}

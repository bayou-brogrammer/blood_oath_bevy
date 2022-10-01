use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
    player_q: Query<(Entity, &mut Position), With<Player>>,
    mut move_events: EventWriter<WantsToMove>,
) {
    let key = keys.get_pressed().next().cloned();
    if let Some(key) = key {
        let mut delta = Coord::new(0, 0);

        match key {
            KeyCode::Left => delta.x -= 1,
            KeyCode::Right => delta.x += 1,
            KeyCode::Down => delta.y += 1,
            KeyCode::Up => delta.y -= 1,
            _ => {}
        }

        // move to new position
        if delta.x != 0 || delta.y != 0 {
            let (player_entity, pos) = player_q.single();
            let destination = **pos + delta;
            move_events.send(WantsToMove(player_entity, destination));
        }

        // reset keyboard, bevys bug when changing states
        keys.reset(key);

        commands.insert_resource(NextState(InGameState::ResolveActions));
    }
}

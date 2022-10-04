use crate::prelude::*;

pub fn player_input(
    mut commands: Commands,
    mut keys: ResMut<Input<KeyCode>>,
    mut move_events: EventWriter<WantsToMove>,
    mut pickup_event: EventWriter<WantsToPickupItem>,

    items_query: Query<(Entity, &Position), (With<Item>, Without<Player>)>,
    player_q: Query<(Entity, &mut Position), With<Player>>,
) {
    let key = keys.get_pressed().next().cloned();
    if let Some(key) = key {
        let mut delta = Coord::new(0, 0);
        let (player_entity, pos) = player_q.single();

        match key {
            KeyCode::Left => delta.x -= 1,
            KeyCode::Right => delta.x += 1,
            KeyCode::Down => delta.y += 1,
            KeyCode::Up => delta.y -= 1,

            // Inventory
            KeyCode::G => match try_pickup_item(**pos, items_query) {
                None => {
                    // bo_logging::Logger::new().append("There is nothing here to pick up.").log()
                }
                Some(item) => {
                    pickup_event.send(WantsToPickupItem(player_entity, item));
                }
            },

            _ => {}
        }

        // move to new position
        if delta.x != 0 || delta.y != 0 {
            let destination = **pos + delta;
            move_events.send(WantsToMove(player_entity, destination));
        }

        // reset keyboard, bevys bug when changing states
        keys.reset(key);

        commands.insert_resource(NextState(InGameState::ResolveActions));
    }
}

fn try_pickup_item(
    player_pos: Coord,
    items_query: Query<(Entity, &Position), (With<Item>, Without<Player>)>,
) -> Option<Entity> {
    for (entity, item_pos) in items_query.iter() {
        if **item_pos == player_pos {
            return Some(entity);
        }
    }

    None
}

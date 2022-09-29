use crate::dungeon_crawl::*;

pub fn use_items(
    mut commands: Commands,
    console: ResMut<Console>,
    mut health_target_query: Query<(Entity, &mut Health)>,
    item_messages: Query<(Entity, &ActivateItem)>,
    healing_query: Query<(Entity, &ProvidesHealing)>,
    mapping_query: Query<(Entity, &ProvidesDungeonMap)>,
    mut maptiles_query: Query<(Entity, &mut Visibility), With<MapTile>>,
    names_query: Query<&Naming>,
) {
    // for every message
    for (message_entity, activated_item) in item_messages.iter() {
        // if it is a healing item
        if let Ok((_, healing)) = healing_query.get(activated_item.item) {
            if let Ok((_, mut health)) = health_target_query.get_mut(activated_item.used_by) {
                // increase health
                health.current = i32::min(health.max, health.current + healing.amount);
                // update gamelog
                let target_char = names_query.get(activated_item.used_by).unwrap();

                console.write(
                    Logger::new()
                        .append(target_char.0.clone())
                        .append("heals")
                        .append(healing.amount)
                        .append("HP.")
                        .to_str(),
                );
            }
        }

        // if it is a map item
        if mapping_query.get(activated_item.item).is_ok() {
            // reveal all tiles
            maptiles_query.iter_mut().for_each(|(_, mut vis)| vis.is_visible = true);
            console.write(Logger::new().append("The Map is revealed!").to_str());
        }

        // delete the message
        commands.entity(message_entity).despawn();
        // remove the item
        commands.entity(activated_item.item).despawn_recursive();
    }
}

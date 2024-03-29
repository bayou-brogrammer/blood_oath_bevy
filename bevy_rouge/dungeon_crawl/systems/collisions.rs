use crate::dungeon_crawl::*;

pub fn collisions(
    mut commands: Commands,
    enemy_positions: Query<(Entity, &Point), With<Enemy>>,
    player_query: Query<&Point, (Changed<Point>, With<Player>)>,
) {
    // get the player position based on the query
    if let Ok(player_position) = player_query.single() {
        // iterate through all the enemies
        enemy_positions
            .iter()
            // if player and enemy are at the same position
            .filter(|(_, pos)| **pos == *player_position)
            // remove the enemy
            .for_each(|(ent, _)| {
                commands.entity(ent).despawn();
            });
    }
}

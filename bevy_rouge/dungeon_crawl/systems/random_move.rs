use crate::dungeon_crawl::*;

pub fn random_move(
    mut commands: Commands,
    movers: Query<(Entity, &Point), With<MovingRandomly>>,
    positions: Query<(Entity, &Point), With<Health>>,
    player: Query<Entity, With<Player>>,
) {
    // for each enemy
    for (ent, pos) in movers.iter() {
        // calculate a random destination
        let destination = match rng.range(0..4) {
            0 => Point { x: -1, y: 0, z: 0 },
            1 => Point { x: 1, y: 0, z: 0 },
            2 => Point { x: 0, y: -1, z: 0 },
            _ => Point { x: 0, y: 1, z: 0 },
        } + *pos;

        if destination != *pos {
            // placeholder to know if movement was an attack or just a movement
            let mut attacked = false;
            // for each entity that can move and has health
            positions
                .iter()
                // get entities at the destination of the movement
                .filter(|(_, target_pos)| **target_pos == destination)
                // for each victim
                .for_each(|(victim, _)| {
                    // if the victim is the player
                    if let Ok(player_ent) = player.get(victim) {
                        // send an attack message
                        commands.spawn().insert(WantsToAttack { attacker: ent, victim: player_ent });
                    }
                    attacked = true;
                });

            if !attacked {
                // move to new position
                commands.spawn().insert(WantsToMove { entity: ent, destination: destination });
            }
        }
    }
}

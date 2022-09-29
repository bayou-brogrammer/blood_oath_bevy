use crate::dungeon_crawl::*;

pub fn combat(
    mut map: ResMut<Map>,
    console: Res<Console>,
    mut commands: Commands,
    names_query: Query<&Naming>,
    player: Query<Entity, With<Player>>,
    attacker_messages: Query<(Entity, &WantsToAttack)>,
    mut health_query: Query<(&mut Health, &Point, &Naming)>,
    damage_query: Query<(&Damage, Option<&Carried>, Option<&Equipped>)>,
) {
    // get the list of victim messages
    let victims: Vec<(Entity, Entity, Entity)> =
        attacker_messages.iter().map(|(entity, attack)| (entity, attack.attacker, attack.victim)).collect();

    // for every message, get the message itself, the attacker and the victim
    victims.iter().for_each(|(message, attacker, victim)| {
        // calculate damage of attack. total damage = base damage + weapon damage
        let base_damage = if let Ok((d, _, _)) = damage_query.get(*attacker) { d.0 } else { 0 };

        let w_damage: i32 = damage_query
            .iter()
            .filter(|(_, c, e)| c.is_some() && e.is_some())
            .map(|(dmg, carried, _)| (dmg, carried.unwrap()))
            .filter(|(_, carried)| carried.0 == *attacker)
            .map(|(dmg, _)| dmg.0)
            .sum();

        let final_damage = base_damage + w_damage;

        // get the victim entity and decrease the hp
        if let Ok((mut hp, pos, name)) = health_query.get_mut(*victim) {
            hp.current -= final_damage;
            // add action to gamelog, first get name of attacker, then build message
            let attacker_char = names_query.get(*attacker).unwrap();

            console.write_color(
                Logger::new()
                    .append(&attacker_char.0)
                    .append("hits")
                    .append(&name.0)
                    .append("for")
                    .append(final_damage)
                    .append("hp.")
                    .to_str(),
                Color::RED,
            );

            // less than 1 HP remove it
            if hp.current < 1 && player.get(*victim).is_err() {
                map.remove_entity(*victim, *pos);
                commands.entity(*victim).despawn();
            }
        }
        // remove the message
        commands.entity(*message).despawn();
    });
}

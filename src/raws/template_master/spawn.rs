use crate::prelude::*;
use bevy::ecs::system::EntityCommands;
use std::collections::HashMap;

pub enum SpawnType {
    // Carried(Entity),
    // Equipped(Entity),
    AtPosition(Coord),
}

#[rustfmt::skip]
pub fn spawn_position(pos: SpawnType, eb: &mut EntityCommands, tag: &str, raws: &RawMaster) {
    // Spawn in the specified location
    match pos {
        SpawnType::AtPosition(coord) => { eb.insert(Position(coord)); }
        // SpawnType::Carried(by) => { eb.insert(InBackpack { owner: by }); }
        // SpawnType::Equipped(by) => {
        //     let slot = find_slot_for_equippable_item(tag, raws);
        //     eb.insert(Equipped::new(by, slot));
        // }
    }
}

pub fn spawn_base_entity<T: BaseRawComponent + Clone>(
    raws: &RawMaster,
    eb: &mut EntityCommands,
    entity_list: &[T],
    indexes: &HashMap<String, usize>,
    key: &str,
    pos: SpawnType,
) -> T {
    let entity_template = &entity_list[indexes[key]];

    // Spawn in the specified location
    spawn_position(pos, eb, key, raws);

    // Renderable
    eb.insert(get_renderable_component(&entity_template.glyph()));

    // Name Component
    eb.insert(Name::new(entity_template.name()));

    entity_template.clone()
}

pub fn spawn_player_from_raw(commands: &mut Commands, coord: Coord) -> Entity {
    let mut player = commands.spawn();
    let raws = RAWS.lock();
    let player_template = raws.templates.player.as_ref().expect("Player not loaded");
    let glyph = get_renderable_component(&player_template.glyph());

    let stats = create_stats_from_raw_stats(&player_template.stats);

    player
        .insert_bundle(PlayerBundle::new(
            coord,
            player_template.name.clone(),
            glyph.glyph,
            glyph.color,
            stats,
        ))
        .insert(WaitingForTurn::default());

    player.id()
}

pub fn spawn_named_hostile(
    raws: &RawMaster,
    commands: &mut Commands,
    key: &str,
    spawn_type: SpawnType,
) -> Option<Entity> {
    let mut eb = commands.spawn();

    let hostile_template = &raws.templates.hostiles[raws.hostile_index[key]];

    let stats = create_stats_from_raw_stats(&hostile_template.stats);
    let glyph = get_renderable_component(&hostile_template.glyph());

    let coord = match spawn_type {
        SpawnType::AtPosition(coord) => coord,
        _ => panic!("Hostiles must be spawned at a position"),
    };

    eb.insert_bundle(HostileBundle::new(coord, key, glyph.glyph, glyph.color, stats))
        .insert(WaitingForTurn::default());

    match hostile_template.ai {
        AIType::Hostile => {
            //         let chase_and_attack =
            //             Steps::build().step(ChasePlayer::default()).step(AttackPlayer::default());
            //         let thinker = Thinker::build()
            //             .picker(FirstToScore { threshold: 0.8 })
            //             .when(
            //                 WinningScorer::build(1.0)
            //                     .push(CanSeePlayer::default())
            //                     .push(SawPlayerRecently::default()),
            //                 chase_and_attack,
            //             )
            //             .otherwise(Meander);
            //         eb.insert(thinker);
        }
    }

    Some(eb.id())
}

pub fn spawn_named_item(
    raws: &RawMaster,
    commands: &mut Commands,
    key: &str,
    spawn_type: SpawnType,
) -> Option<Entity> {
    let mut eb = commands.spawn();
    let item_template = &raws.templates.items[raws.item_index[key]];
    let glyph = get_renderable_component(&item_template.glyph());

    match spawn_type {
        SpawnType::AtPosition(coord) => {
            eb.insert_bundle(ItemBundle::new(coord, key, glyph.glyph, glyph.color));
        }
    }

    Some(eb.id())
}

pub fn spawn_named_entity(
    commands: &mut Commands,
    key: &str,
    pos: SpawnType,
) -> Option<Entity> {
    let raws = RAWS.lock();
    if raws.hostile_index.contains_key(key) {
        return spawn_named_hostile(&raws, commands, key, pos);
    } else if raws.item_index.contains_key(key) {
        return spawn_named_item(&raws, commands, key, pos);
    }

    None
}

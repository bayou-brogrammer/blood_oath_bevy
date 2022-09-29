use crate::dungeon_crawl::*;
use bevy::app::AppExit;

#[derive(Debug)]
pub enum PlayerInputResult {
    AppQuit,
    _Descend,
    TurnDone,
    NoResult,
    // Inventory
    _ShowDrop,
    _ShowRemove,
    _ShowInventory,
    _ShowInventoryShortcut,
}

pub fn player_input(
    console: Res<Console>,
    mut commands: Commands,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut evs: EventWriter<WantsToMove>,
    // Queries
    enemies: Query<(Entity, &Point), With<Enemy>>,
    items: Query<(Entity, &Point, &Naming), With<Item>>,
    mut player_query: Query<(Entity, &mut Player, &Point, &mut TextureAtlasSprite), With<Player>>,
) -> PlayerInputResult {
    let (player_ent, mut player, pos, mut sprite) = player_query.single_mut();
    let mut action = true;
    let mut wait = false;

    let mut new_position = *pos;

    let key = keyboard_input.get_pressed().next().cloned();
    if let Some(key) = key {
        match key {
            KeyCode::Escape => return PlayerInputResult::AppQuit,
            KeyCode::Left => {
                new_position.x -= 1;
                player.facing = Facing::Left;
            }
            KeyCode::Right => {
                new_position.x += 1;
                player.facing = Facing::Right;
            }
            KeyCode::Down => {
                new_position.y -= 1;
                player.facing = Facing::Down;
            }
            KeyCode::Up => {
                new_position.y += 1;
                player.facing = Facing::Up;
            }
            KeyCode::G => {
                // Grab item at this position
                items.iter().filter(|(_, item_pos, _)| **item_pos == *pos).for_each(|(item_ent, _, name)| {
                    // remove render info and add carried component
                    commands
                        .entity(item_ent)
                        .remove_bundle::<SpriteSheetBundle>()
                        .insert(Carried(player_ent));

                    console.write(Logger::new().append("You grab a").append(name.0.clone()).to_str());
                });
            }
            KeyCode::I => {
                // turn_state.push(TurnState::InventoryPopup).unwrap();
                action = false;
            }
            KeyCode::E => {
                // turn_state.push(TurnState::EquipmentPopup).unwrap();
                action = false;
            }
            _ => wait = true,
        }

        match player.facing {
            Facing::Left => sprite.index = 4,
            Facing::Right => sprite.index = 8,
            Facing::Up => sprite.index = 12,
            Facing::Down => sprite.index = 0,
        }

        // move to new position
        if new_position != *pos {
            // placeholder to know if it just a move or an attack
            let mut hit_something = false;
            // check if there is an enemy at the destination position
            enemies
                .iter()
                .filter(|(_, pos)| **pos == new_position)
                // if there's an enemy, say you hit something and send a WantsToAttack
                .for_each(|(victim, _)| {
                    hit_something = true;

                    // commands.spawn().insert(WantsToAttack { attacker: player_ent, victim });
                });

            // if it did not hit then it is just a movement
            if !hit_something {
                evs.send(WantsToMove(player_ent, *pos, new_position));
                // commands.spawn().insert(WantsToMove { entity: player_ent, destination: new_position });
            }
        } else if wait {
            console.write(Logger::new().append("You wait...").to_str());
        }

        // reset keyboard, bevys bug when changing states
        keyboard_input.reset(key);

        if action {
            // update state
            return PlayerInputResult::TurnDone;
        }
    }

    PlayerInputResult::NoResult
}

pub fn player_result(
    In(result): In<PlayerInputResult>,
    mut commands: Commands,
    mut exit: EventWriter<AppExit>,
) {
    match result {
        PlayerInputResult::NoResult => {}
        PlayerInputResult::AppQuit => exit.send(AppExit),
        PlayerInputResult::TurnDone => {
            commands.insert_resource(NextState(AppState::DungeonCrawl(TurnState::Ticking)))
        }
        _ => println!("No result"),
    }
}

// If this is the first weapon we grab, also equip it
pub fn equip_first_weapon(
    mut commands: Commands,
    weapons_added: Query<Entity, (With<Weapon>, Added<Carried>)>,
    total_carried_weapons: Query<Entity, (With<Weapon>, With<Carried>)>,
) {
    for entity in weapons_added.iter() {
        // if we only have 1 weapon, equip it too
        if total_carried_weapons.iter().count() == 1 {
            commands.entity(entity).insert(Equipped);
        }
    }
}

// If this is the first weapon we grab, also equip it
pub fn equip_weapon_log(
    console: Res<Console>,
    equipped_weapon: Query<(Entity, &Naming), (With<Weapon>, With<Carried>, Added<Equipped>)>,
) {
    for (_, name) in equipped_weapon.iter() {
        console.write_color(Logger::new().append("You equip").append(name.0.clone()).to_str(), Color::CYAN);
    }
}

pub struct PlayerInputPlugin;
impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .label("input")
                .run_in_state(AppState::DungeonCrawl(TurnState::AwaitingInput))
                // .run_if_resource_equals(TurnState::AwaitingInput)
                .with_system(player_input.chain(player_result))
                .with_system(equip_first_weapon)
                .with_system(equip_weapon_log)
                .into(),
        );
    }
}

use crate::dungeon_crawl::*;

mod random_table;
pub use random_table::*;

pub fn spawn_player(mb: Res<MapBuilder>, mut commands: Commands, textures: Res<TextureAssets>) {
    let start_pos = mb.starting_position.unwrap();
    println!("Spawning player at {:?}", start_pos);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: get_sprite(0),
            texture_atlas: textures.rogue.clone(),
            transform: Transform::from_translation(Vec3::Z * ZBUF_PLAYER),
            ..Default::default()
        })
        .insert(start_pos)
        .insert(TileSize::square(1.0))
        .insert(Health { current: 10, max: 20 })
        .insert(Player { map_level: 0, facing: Facing::Down })
        .insert(Naming("Player".to_string()))
        .insert(FieldOfView::new(8))
        .insert(Damage(1));
}

pub fn spawn_level(
    mb: Res<MapBuilder>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    master: Res<TemplateMaster>,
) {
    for (spawn_idx, spawn_key) in mb.spawn_list.iter() {
        let spawn_pt = mb.map.index_to_point2d(*spawn_idx);

        let spawn_result =
            master.spawn_named_entity(&mut commands, &textures, spawn_key, SpawnType::AtPosition(spawn_pt));
        if spawn_result.is_some() {
            continue;
        }

        println!("WARNING: We don't know how to spawn [{}]!", spawn_key);
    }
}

// pre_advance level requires to delete all entities, except the player their items
// set the field of view to dirty so it is re-calculated
fn pre_advance_level(
    mut commands: Commands,
    position_q: Query<Entity, (With<Point>, Without<Player>, Without<Carried>)>,
    mut fov_q: Query<&mut FieldOfView>,
) {
    // remove all the entities with position component except player
    for e in position_q.iter() {
        commands.entity(e).despawn_recursive();
    }

    // set all the fov is_dirty to true, so they will need to be recalculated
    fov_q.iter_mut().for_each(|mut fov| fov.is_dirty = true);
}

// post_advance level sets the location of the player in the new map, advaces its level var
fn post_advance_level(mb: Res<MapBuilder>, mut player_q: Query<(&mut Point, &mut Player)>) {
    let player_start = mb.starting_position;
    let (mut pos, mut player) = player_q.single_mut();
    player.map_level += 1;
    *pos = player_start.unwrap();
}

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        // Start Level
        app.add_exit_system_set(
            AppState::MapGen,
            ConditionSet::new()
                .label(SystemLabels::SpawnEntities)
                .with_system(spawn_player)
                .with_system(spawn_level)
                .into(),
        );

        app.add_enter_system(AppState::NextLevel, pre_advance_level).add_exit_system_set(
            AppState::NextLevel,
            ConditionSet::new().with_system(post_advance_level).with_system(spawn_level).into(),
        );

        // End Level
        app.add_enter_system(AppState::GameOver, despawn_all_with::<Point>)
            .add_enter_system(AppState::Victory, despawn_all_with::<Point>);
    }
}

use crate::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    tile_map: Res<TileMap>,
    map_builder: Res<MapBuilder>,
    textures: Res<TextureAssets>,
) {
    let start_pos = &map_builder.player_start;

    let player_entity = commands
        .spawn_bundle(PlayerBundle::new(**start_pos, textures.ascii_tileset_atlas.clone()))
        .insert(WaitingForTurn::default())
        .id();

    // Add a 2D Camera
    let (camera_x, camera_y) = get_camera_target(**start_pos, &tile_map);
    let transform = Transform {
        translation: Vec3::new(camera_x, camera_y, 999.0),
        scale: Vec3::new(0.6, 0.6, 1.0),
        ..Default::default()
    };
    commands
        .spawn_bundle(Camera2dBundle { transform, ..Default::default() })
        .insert(CameraFollow(player_entity));
}

pub fn spawn_others(
    mut commands: Commands,
    map_builder: Res<MapBuilder>,
    textures: Res<TextureAssets>,
) {
    map_builder.rooms.iter().skip(1).map(Rect::center).for_each(|pt| {
        commands.spawn_bundle(EnemyBundle::new(pt, textures.ascii_tileset_atlas.clone()));
    });
}

//////////////////////////////////////////////////////////////////////////////////////////

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system_set(
            GameState::Setup,
            SystemSet::new().with_system(spawn_player).with_system(spawn_others),
        );
    }
}

use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////
/// Render Utility
////////////////////////////////////////////////////////////////////////////////

pub fn convert_pos(pos: f32, tile_size: f32, board_size: f32) -> f32 {
    (pos as f32 * tile_size) + (tile_size / 2.) - board_size
}

pub fn position_translation(mut query: Query<(&mut Transform, &Point), Changed<Point>>) {
    for (mut transform, grid_position) in query.iter_mut() {
        transform.translation = Vec3::new(
            convert_pos(grid_position.x as f32, 16., MAP_SIZE),
            convert_pos(grid_position.y as f32, 16., MAP_SIZE),
            transform.translation.z,
        );
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct MainCamera;

fn setup_game_camera(mut commands: Commands) {
    // Add a 2D Camera
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(0.5, 0.5, 1.0),
                translation: Vec3::new(0.0, 0.0, 999.0),
                ..default()
            },
            ..Default::default()
        })
        .insert(MainCamera);
}

pub fn camera_position(
    mut player: Query<&Point, (Changed<Point>, With<Player>, Without<Camera>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let position = match player.get_single_mut() {
        Ok(position) => *position,
        Err(_) => return,
    };
    let mut camera = camera.single_mut();

    let cam_x = convert_pos(position.x as f32, 16., MAP_SIZE);
    let cam_y = convert_pos(position.y as f32, 16., MAP_SIZE);
    camera.translation = Vec3::new(cam_x, cam_y, 999.0);
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_game_camera);

        app.add_system(camera_position.label(SystemLabels::CameraMove).after(SystemLabels::MoveEntity))
            .add_system(position_translation.after(SystemLabels::CameraMove));
    }
}

use crate::prelude::*;

////////////////////////////////////////////////////////////////////////////////
/// Render Utility
////////////////////////////////////////////////////////////////////////////////

pub fn move_sprites(mut query: Query<(&mut Transform, &Point), Changed<Point>>) {
    for (mut transform, pos_pt) in query.iter_mut() {
        let (target_x, target_y) = pt_spritecoords(*pos_pt);
        transform.translation.x = target_x;
        transform.translation.y = target_y;
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

    let (target_x, target_y) = pt_spritecoords(position);
    camera.translation = Vec3::new(target_x, target_y, 999.0);
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_game_camera);

        app.add_system(camera_position.label(SystemLabels::CameraMove).after(SystemLabels::MoveEntity))
            .add_system(position_translation.after(SystemLabels::CameraMove));
    }
}

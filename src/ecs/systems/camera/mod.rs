use crate::prelude::*;

mod move_sprites;
pub use move_sprites::*;

pub fn get_camera_target(pos: Point, map: &TileMap) -> (f32, f32) {
    let (mut target_x, mut target_y) = pt_spritecoords(pos);

    let map_width_px = TILE_SIZE * map.width as f32;
    let map_height_px = TILE_SIZE * map.height as f32;

    let camera_min_x = (VIEWPORT_WIDTH / 2.0) - (VIEWPORT_OFFSET.0 / 2.0);
    let camera_max_x = map_width_px - (VIEWPORT_WIDTH / 2.0) - (VIEWPORT_OFFSET.0 / 2.0);
    let camera_min_y = (VIEWPORT_HEIGHT / 2.0) - (VIEWPORT_OFFSET.1 / 2.0);
    let camera_max_y = map_height_px - (VIEWPORT_HEIGHT / 2.0) - (VIEWPORT_OFFSET.1 / 2.0);

    if target_x < camera_min_x {
        target_x = camera_min_x;
    } else if target_x > camera_max_x {
        target_x = camera_max_x;
    }

    if target_y < camera_min_y {
        target_y = camera_min_y;
    } else if target_y > camera_max_y {
        target_y = camera_max_y;
    }

    (target_x, target_y)
}

///////////////////////////////////////////////////////////////////////////////////////////////
// Positioning
///////////////////////////////////////////////////////////////////////////////////////////////

pub fn update_camera(
    map: Res<TileMap>,
    position_q: Query<&Position>,
    mut camera_q: Query<(&mut Transform, &CameraFollow), With<Camera>>,
) {
    let (mut transform, camera_target) = camera_q.single_mut();
    if let Ok(Position(target_pt)) = position_q.get(camera_target.0) {
        let (target_x, target_y) = get_camera_target(*target_pt, map.as_ref());
        transform.translation.x += target_x - transform.translation.x;
        transform.translation.y += target_y - transform.translation.y;
    }
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // Always when InGame
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_not_in_state(InGameState::Dead)
                .with_system(move_sprites)
                .with_system(update_camera)
                .into(),
        );
    }
}

use crate::prelude::*;

pub fn move_sprites(
    map: Res<TileMap>,
    mut query: Query<(&mut Transform, &Position), Changed<Position>>,
) {
    for (mut transform, pos_pt) in query.iter_mut() {
        if map.in_bounds(**pos_pt) {
            let (target_x, target_y) = pt_spritecoords(**pos_pt);
            transform.translation.x = target_x;
            transform.translation.y = target_y;
            transform.translation.z = MOB_Z;
        }
    }
}

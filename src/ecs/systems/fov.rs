use crate::prelude::*;

pub fn fov(
    mut map: ResMut<TileMap>,
    mut fov_q: Query<(&Position, &mut FieldOfView, Option<&Player>)>,
    mut tile_q: Query<(&mut TileVisible, &mut TileColor, &TilePos)>,
    mut vis_q: Query<(&Position, &mut Visibility), Without<Player>>,
) {
    for (Position(pos_pt), mut fov, player) in fov_q.iter_mut() {
        fov.visible_tiles = field_of_view_set(pos_pt.to_point(), fov.radius as i32, &*map);

        if player.is_some() {}
    }
}

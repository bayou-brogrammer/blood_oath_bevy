use crate::prelude::*;

pub fn fov(
    mut map: ResMut<TileMap>,
    mut fov_q: Query<(&Position, &mut FieldOfView, Option<&Player>)>,
) {
    for (Position(pos_pt), mut fov, player) in fov_q.iter_mut() {
        fov.is_dirty = false;
        fov.visible_tiles = field_of_view_set(pos_pt.to_point(), fov.radius as i32, &*map);

        if player.is_some() {
            map.clear_visible();

            fov.visible_tiles.iter().for_each(|pt| {
                map.set_revealed_and_visible(pt.to_coord());
            });
        }
    }
}

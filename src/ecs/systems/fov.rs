use crate::prelude::*;

pub fn fov(
    mut map: ResMut<TileMap>,
    mut fov_q: Query<(&Position, &mut FieldOfView, Option<&Player>)>,
    mut tile_q: Query<(&mut TileVisible, &mut TileColor, &TilePos)>,
    mut vis_q: Query<(&Position, &mut Visibility), Without<Player>>,
) {
    for (Position(pos_pt), mut fov, player) in fov_q.iter_mut() {
        fov.visible_tiles = field_of_view_set(*pos_pt, fov.radius as i32, &*map);

        if player.is_some() {
            for (mut tile_vis, mut tile_color, tile_pos) in tile_q.iter_mut() {
                let pt = tile_pos.to_point();
                let idx = map.point2d_to_index(pt);

                if fov.visible_tiles.contains(&pt) {
                    // reveal tiles
                    tile_vis.0 = true;
                    tile_color.0.set_a(1.0);
                    map.tiles[idx].flags |= TileFlags::IN_VIEW;
                } else if tile_vis.0 {
                    // fade revealed tiles that are not visible
                    tile_color.0.set_a(0.1);
                    map.tiles[idx].flags |= TileFlags::EXPLORED;
                }
            }

            for (Position(e_pos_pt), mut e_vis) in vis_q.iter_mut() {
                if fov.visible_tiles.contains(e_pos_pt) {
                    e_vis.is_visible = true;
                } else {
                    e_vis.is_visible = false;
                }
            }
        }
    }
}

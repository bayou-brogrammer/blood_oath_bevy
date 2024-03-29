use crate::dungeon_crawl::*;

pub fn fov(mb: Res<MapBuilder>, mut views_query: Query<(&Point, &mut FieldOfView)>) {
    views_query.iter_mut().filter(|(_, fov)| fov.is_dirty).for_each(|(pos, mut fov)| {
        fov.visible_tiles = field_of_view_set(*pos, fov.radius, &mb.map);
        fov.is_dirty = false;
    });
}

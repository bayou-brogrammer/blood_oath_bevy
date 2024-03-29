use super::*;

pub fn map_indexing(
    mut map: ResMut<Map>,
    dead_q: Query<Entity>,
    blocking_q: Query<(Entity, &Point, Option<&BlocksTile>)>,
) {
    if blocking_q.is_empty() && dead_q.is_empty() {
        return;
    }

    map.clear();
    map.populate_blocked();
    // crate::spatial::populate_opaque_from_map(&map);

    for (entity, pos, blocker) in blocking_q.iter() {
        let idx = map.point2d_to_index(*pos);
        map.index_entity(entity, idx, blocker.is_some());
    }
}

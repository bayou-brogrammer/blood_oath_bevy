use crate::prelude::*;

pub fn map_indexing(
    mut map: ResMut<TileMap>,
    blocking_q: Query<(Entity, &Position, Option<&BlocksMovement>, Option<&BlocksVisibility>)>,
) {
    map.clear();
    map.populate_blocked();
    map.populate_opaque();

    for (entity, Position(pos_pt), blocker, opaque) in blocking_q.iter() {
        let idx = map.point2d_to_index(*pos_pt);
        map.index_entity(entity, idx, blocker.is_some(), opaque.is_some());
    }
}

use crate::dungeon_crawl::*;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct BlocksMovement;

#[derive(Component)]
pub struct BlocksVision;

pub fn update_world_map(
    mut world: ResMut<Map>,
    t: Query<(&Tile, &BlocksMovement)>,
    m: Query<&BlocksMovement>,
    v: Query<&BlocksVision>,
) {
    world.populate_blocked();
}

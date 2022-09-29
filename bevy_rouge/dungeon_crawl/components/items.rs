use crate::dungeon_crawl::*;

#[derive(Component)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Component)]
pub struct ProvidesDungeonMap;

#[derive(Component)]
pub struct Carried(pub Entity);

#[derive(Component)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity,
}

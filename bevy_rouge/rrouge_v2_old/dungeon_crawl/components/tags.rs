use crate::dungeon_crawl::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component)]
pub struct Player {
    pub map_level: i32,
    pub facing: Facing,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MapTile;

#[derive(Component)]
pub struct Item;

#[derive(Component)]
pub struct AmuletOfYala;

#[derive(Component, Debug)]
pub struct BlocksTile;

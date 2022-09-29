use crate::dungeon_crawl::*;

#[derive(Component, Clone)]
pub struct Naming(pub String);

// used for objects and similar to provide a description about themselves
#[derive(Component, Clone)]
pub struct Description(pub String);

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

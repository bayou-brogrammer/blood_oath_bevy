use crate::world_gen::*;

mod simple_map;
use simple_map::*;

#[derive(Debug, Clone, Copy)]
pub enum MapToBuild {
    Rooms,
}

pub trait MapBuilder {
    fn get_map(&self) -> Map;
    fn starting_pos(&self) -> Point;
    fn build_map(&mut self, rng: &RandomNumbers);
    fn spawn_entities(&mut self, ecs: &mut World);
}

pub fn random_builder(width: i32, height: i32, new_depth: i32) -> Box<dyn MapBuilder> {
    // Note that until we have a second map type, this isn't even slighlty random
    Box::new(SimpleMapBuilder::new(width, height, new_depth))
}

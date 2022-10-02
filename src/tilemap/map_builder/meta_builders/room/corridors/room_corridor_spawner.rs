use crate::prelude::*;

pub struct CorridorSpawner {}

impl MapArchitect for CorridorSpawner {
    fn build_map(&mut self, build_data: &mut MapBuilder, _rng: &RandomNumbers) {
        self.build(build_data);
    }
}

impl CorridorSpawner {
    pub fn new() -> Box<CorridorSpawner> {
        Box::new(CorridorSpawner {})
    }

    fn build(&mut self, build_data: &mut MapBuilder) {
        if let Some(corridors) = &build_data.corridors {
            for _c in corridors.iter() {
                // todo!()
            }
        } else {
            panic!("Corridor Based Spawning only works after corridors have been created");
        }
    }
}

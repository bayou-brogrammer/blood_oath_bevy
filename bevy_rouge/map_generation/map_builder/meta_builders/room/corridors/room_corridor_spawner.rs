use super::*;

pub struct CorridorSpawner {
    master: TemplateMaster,
}

impl MapArchitect for CorridorSpawner {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        self.build(builder, rng);
    }
}

impl CorridorSpawner {
    pub fn new(master: &mut TemplateMaster) -> Box<CorridorSpawner> {
        Box::new(CorridorSpawner { master: master.clone() })
    }

    fn build(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        if let Some(corridors) = &builder.corridors {
            let mut spawn_corridors = corridors.to_vec();
            spawn_corridors.iter_mut().for_each(|c| builder.spawn_region(c, &mut self.master, rng));
        } else {
            panic!("Corridor Based Spawning only works after corridors have been created");
        }
    }
}

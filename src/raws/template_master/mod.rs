use crate::prelude::*;
use bevy::utils::HashMap;
use std::collections::HashSet;

mod helpers;
mod spawn;

pub use helpers::*;
pub use spawn::*;

pub struct RawMaster {
    templates: Templates,
    item_index: HashMap<String, usize>,
    hostile_index: HashMap<String, usize>,
}

impl RawMaster {
    pub fn empty() -> RawMaster {
        RawMaster {
            item_index: HashMap::new(),
            hostile_index: HashMap::new(),
            templates: Templates {
                player: None,
                items: Vec::new(),
                hostiles: Vec::new(),
                spawn_tables: HashMap::new(),
            },
        }
    }
}

impl RawMaster {
    pub fn load(&mut self, raws: Templates) {
        self.templates = raws;
        let mut used_names: HashSet<String> = HashSet::new();

        // Hostiles
        load_entity_data(&self.templates.items, &mut self.item_index, &mut used_names);
        load_entity_data(&self.templates.hostiles, &mut self.hostile_index, &mut used_names);
    }
}

pub fn load_entity_data<T: 'static + BaseRawComponent>(
    raws: &[T],
    entity_index: &mut HashMap<String, usize>,
    used_names: &mut HashSet<String>,
) {
    for (i, entity) in raws.iter().enumerate() {
        let entity_name = entity.name();

        if used_names.contains(&entity_name) {
            println!("WARNING - duplicate entity name in raws [{}]", entity_name);
        }

        entity_index.insert(entity_name.clone(), i);
        used_names.insert(entity_name.clone());
    }
}

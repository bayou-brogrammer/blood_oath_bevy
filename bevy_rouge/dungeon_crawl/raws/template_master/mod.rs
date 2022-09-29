use super::*;
use std::collections::{HashMap, HashSet};

mod load;
mod spawn;

pub use load::*;
pub use spawn::*;

#[derive(Debug, Clone)]
pub struct TemplateMaster {
    pub templates: Templates,
    pub mob_index: HashMap<String, usize>,
    pub item_index: HashMap<String, usize>,
}

impl TemplateMaster {
    pub fn new(templates: Templates) -> TemplateMaster {
        let mut master = TemplateMaster { mob_index: HashMap::new(), item_index: HashMap::new(), templates };
        master.load();
        master
    }

    #[allow(dead_code)]
    pub fn empty() -> TemplateMaster {
        TemplateMaster { mob_index: HashMap::new(), item_index: HashMap::new(), templates: Templates::new() }
    }

    pub fn get_spawn_table_for_depth(&self, level: i32) -> MasterTable {
        let available_options: Vec<&SpawnTableEntry> = self
            .templates
            .spawn_table
            .iter()
            .filter(|a| level >= a.min_level && level <= a.max_level)
            .collect();

        let mut rt = MasterTable::new();
        for e in available_options.iter() {
            let mut weight = e.weight;

            if e.add_map_depth_to_weight.is_some() {
                weight += level;
            }

            rt.add(e.name.clone(), weight, self);
        }

        rt
    }
}

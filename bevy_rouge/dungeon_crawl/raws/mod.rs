use crate::dungeon_crawl::*;

use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

mod template_master;
mod templates;

pub use template_master::*;
pub use templates::*;

#[derive(Default, Clone, Deserialize, Debug)]
pub struct Templates {
    pub mobs: Vec<MobTemplate>,
    pub items: Vec<ItemTemplate>,
    pub spawn_table: Vec<SpawnTableEntry>,
}

impl Templates {
    pub fn new() -> Self {
        Templates { mobs: Vec::new(), items: Vec::new(), spawn_table: Vec::new() }
    }

    pub fn load(mut commands: Commands) {
        let mob_file = File::open("assets/raws/mobs.ron").expect("Failed opening mob file.");
        let item_file = File::open("assets/raws/items.ron").expect("Failed opening item file.");
        let spawn_file = File::open("assets/raws/spawn_table.ron").expect("Failed opening spawn file.");

        let mobs: Vec<MobTemplate> = from_reader(mob_file).expect("Unable to load mob templates.");
        let items: Vec<ItemTemplate> = from_reader(item_file).expect("Unable to load item templates.");
        let spawn_table: Vec<SpawnTableEntry> =
            from_reader(spawn_file).expect("Unable to load spawn templates.");

        commands.insert_resource(TemplateMaster::new(Templates { items, mobs, spawn_table }));
    }
}

pub struct TemplatePlugin;
impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Templates::load);
    }
}

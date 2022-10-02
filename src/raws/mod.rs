use bevy::utils::HashMap;
use bracket_embedding::prelude::*;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use ron::de::from_bytes;

mod template_master;
mod templates;

pub use template_master::*;
pub use templates::*;

lazy_static! {
    pub static ref RAWS: Mutex<RawMaster> = Mutex::new(RawMaster::empty());
}

#[derive(Debug)]
pub struct Templates {
    pub items: Vec<RawItem>,
    pub hostiles: Vec<RawHostile>,
    pub player: Option<RawPlayer>,
    pub spawn_tables: HashMap<String, Vec<SpawnTableEntry>>,
}

impl Templates {
    fn load_raw<'a, T: serde::Deserialize<'a>>(raw_data: &'static [u8]) -> T {
        // Retrieve the raw data as an array of u8 (8-bit unsigned chars)
        match from_bytes::<T>(raw_data) {
            Ok(template) => template,
            Err(e) => panic!("Unable to load template: {}", e),
        }
    }

    fn _load_file<'a, T: serde::Deserialize<'a>>(file_path: &str) -> T {
        // Retrieve the raw data as an array of u8 (8-bit unsigned chars)
        let raw_data = EMBED.lock().get_resource(file_path.to_string()).unwrap();
        match from_bytes::<T>(raw_data) {
            Ok(template) => template,
            Err(e) => panic!("Unable to load template: {}", e),
        }
    }
}

embedded_resource!(RAW_PLAYER_FILE, "../../assets/raws/player.ron");
embedded_resource!(RAW_ITEMS_FILE, "../../assets/raws/items.ron");
embedded_resource!(RAW_HOSTILE_FILE, "../../assets/raws/hostiles.ron");
embedded_resource!(RAW_SPAWN_TABLE_FILE, "../../assets/raws/spawn_tables.ron");

pub fn load_raws() {
    link_resource!(RAW_ITEMS_FILE, "assets/raws/items.ron");
    link_resource!(RAW_HOSTILE_FILE, "assets/raws/mobs.ron");
    link_resource!(RAW_PLAYER_FILE, "assets/raws/player.ron");
    link_resource!(RAW_SPAWN_TABLE_FILE, "assets/raws/spawn_tables.ron");

    let player = Templates::load_raw::<RawPlayer>(RAW_PLAYER_FILE);
    let hostiles = Templates::load_raw::<Vec<RawHostile>>(RAW_HOSTILE_FILE);
    let items = Templates::load_raw::<Vec<RawItem>>(RAW_ITEMS_FILE);

    let mut spawn_tables = HashMap::new();
    for spawn_table in Templates::load_raw::<Vec<SpawnTable>>(RAW_SPAWN_TABLE_FILE) {
        spawn_tables.insert(spawn_table.name.to_string(), spawn_table.entries.clone());
    }

    RAWS.lock().load(Templates { player: Some(player), spawn_tables, hostiles, items });
}

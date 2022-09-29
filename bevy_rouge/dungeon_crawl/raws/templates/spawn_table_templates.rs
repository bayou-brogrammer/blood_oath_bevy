use super::*;

#[derive(Deserialize, Debug, Clone)]
pub struct SpawnTableEntry {
    pub weight: i32,
    pub name: String,
    pub min_level: i32,
    pub max_level: i32,
    pub add_map_depth_to_weight: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpawnTable(pub Vec<SpawnTableEntry>);

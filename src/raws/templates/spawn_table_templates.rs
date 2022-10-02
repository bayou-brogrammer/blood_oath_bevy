use crate::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct SpawnTableEntry {
    pub weight: i32,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpawnTable {
    pub name: String,
    pub entries: Vec<SpawnTableEntry>,
}

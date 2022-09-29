use crate::impl_raw;

use super::*;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Clone, Deserialize, Debug)]
pub struct ItemTemplates {
    pub entities: Vec<ItemTemplate>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ItemTemplate {
    pub name: String,
    pub frequency: i32,
    pub glyph: GlyphTemplate,
    pub levels: HashSet<usize>,
    pub base_damage: Option<i32>,
    pub description: Option<String>,
    pub provides: Option<Vec<(String, i32)>>,
}

impl_raw!(ItemTemplate);

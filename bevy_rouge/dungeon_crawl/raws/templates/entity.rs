use crate::impl_raw;

use super::*;
use std::collections::HashSet;

#[derive(Deserialize, Debug, Clone)]
pub enum AIType {
    Basic,
    Bystander,
}

#[derive(Clone, Deserialize, Debug)]
pub struct MobTemplate {
    pub name: String,
    pub frequency: i32,
    pub hp: Option<i32>,
    pub blocks_tile: bool,
    pub vision_range: i32,
    pub ai: Option<AIType>,
    pub glyph: GlyphTemplate,
    pub levels: HashSet<usize>,
    pub base_damage: Option<i32>,
    pub description: Option<String>,
}

impl_raw!(MobTemplate);

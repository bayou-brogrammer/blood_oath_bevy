use crate::prelude::*;

use crate::impl_raw;

#[derive(Deserialize, Debug, Clone)]
pub struct RawHostile {
    pub ai: AIType,
    pub name: String,
    pub stats: RawStats,
    pub glyph: RawGlyph,
    pub vision_range: i32,
}

impl_raw!(RawHostile);

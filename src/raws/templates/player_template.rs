use crate::prelude::*;

use crate::impl_raw;

#[derive(Deserialize, Debug, Clone)]
pub struct RawPlayer {
    pub name: String,
    pub stats: RawStats,
    pub vision_range: i32,
    pub glyph: RawGlyph,
}

impl_raw!(RawPlayer);

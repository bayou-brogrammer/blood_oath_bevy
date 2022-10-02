use crate::impl_raw;
use crate::prelude::*;

#[derive(Deserialize, Debug, Clone)]
pub struct RawItem {
    pub name: String,
    pub glyph: RawGlyph,
}

impl_raw!(RawItem);

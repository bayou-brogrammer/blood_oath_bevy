use crate::{impl_new, prelude::*};

#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RenderOrder {
    Particle, // Top
    Actor,
    Item,
    Corpse, // Last
}

#[derive(Component, Clone)]
pub struct Glyph {
    pub color: ColorPair,
    pub glyph: FontCharType,
    pub render_order: RenderOrder,
}

impl_new!(Glyph, glyph: FontCharType, color: ColorPair, render_order: RenderOrder);

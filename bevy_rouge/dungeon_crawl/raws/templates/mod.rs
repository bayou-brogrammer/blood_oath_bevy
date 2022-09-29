use serde::Deserialize;

mod entity;
mod item;
mod spawn_table_templates;

pub use entity::*;
pub use item::*;
pub use spawn_table_templates::*;

#[repr(u16)]
#[derive(Deserialize, Debug, Clone)]
pub enum RenderOrder {
    Tile,
    Item,
    Creature,
    Player,
}

impl From<RenderOrder> for f32 {
    fn from(order: RenderOrder) -> Self { order as u16 as f32 }
}

#[derive(Deserialize, Debug, Clone)]
pub enum Glyph {
    Char(char),
    Index(usize),
}

#[derive(Deserialize, Debug, Clone)]
pub struct GlyphTemplate {
    pub glyph: Glyph,
    pub color: String,
    pub order: RenderOrder,
}

#[derive(Clone, Deserialize, Debug, Eq, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

pub trait BaseRawComponent: std::fmt::Debug + Clone {
    fn name(&self) -> String;
    fn description(&self) -> Option<String>;
    fn glyph(&self) -> &GlyphTemplate;
}

#[macro_export]
macro_rules! impl_raw {
    ($to:ty) => {
        impl BaseRawComponent for $to {
            fn name(&self) -> String { self.name.clone() }
            fn description(&self) -> Option<String> { self.description.clone() }
            fn glyph(&self) -> &GlyphTemplate { &self.glyph }
        }
    };
}

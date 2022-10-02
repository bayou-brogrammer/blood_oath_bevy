use crate::prelude::*;

use core::fmt::Debug;
use std::any::Any;

mod hostile_templates;
mod item_templates;
mod player_template;
mod spawn_table_templates;

pub use hostile_templates::*;
pub use item_templates::*;
pub use player_template::*;
pub use spawn_table_templates::*;

pub trait BaseRawComponent: Debug + Clone {
    fn name(&self) -> String;
    fn glyph(&self) -> RawGlyph;
    fn as_any(&self) -> &dyn Any;
}

#[macro_export]
macro_rules! impl_raw {
    ($to:ty) => {
        impl BaseRawComponent for $to {
            fn name(&self) -> String {
                self.name.clone()
            }
            fn glyph(&self) -> RawGlyph {
                self.glyph.clone()
            }
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    };
}

#[derive(Deserialize, Debug, Clone)]
pub enum AIType {
    Hostile,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawStats {
    pub max_hp: Option<i32>,
    pub base_armor: Option<i32>,
    pub magic_resistance: Option<i32>,
    pub physical_resistance: Option<i32>,
    pub movement_cost: Option<u32>,
    pub unarmed_damage: Option<(u32, u32)>,
    pub unarmed_attack_cost: Option<u32>,
    pub chance_to_hit: Option<f32>,
    pub chance_to_evade: Option<f32>,
    pub chance_to_crit: Option<f32>,
    pub crit_dmg_modifier: Option<f32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawGlyph {
    pub char: char,
    pub order: RenderOrder,
    pub fg: (u8, u8, u8),
    pub bg: Option<(u8, u8, u8)>,
}

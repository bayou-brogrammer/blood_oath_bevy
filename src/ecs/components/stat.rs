use crate::prelude::*;

pub struct Pool {
    pub max: i32,
    pub current: i32,
}

#[derive(Component)]
pub struct Stats {
    pub hp: Pool,
    pub base_armor: i32,
    pub magic_resistance: i32,
    pub physical_resistance: i32,
    pub movement_cost: u32,
    pub unarmed_damage: (u32, u32),
    pub unarmed_attack_cost: u32,
    pub chance_to_hit: f32,
    pub chance_to_evade: f32,
    pub chance_to_crit: f32,
    pub crit_dmg_modifier: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            hp: Pool { current: 100, max: 100 },
            base_armor: 0,
            magic_resistance: 0,
            physical_resistance: 0,
            movement_cost: 100,
            unarmed_attack_cost: 100,
            unarmed_damage: (10, 25),
            chance_to_hit: 0.75,
            chance_to_evade: 0.10,
            chance_to_crit: 0.05,
            crit_dmg_modifier: 1.5,
        }
    }
}

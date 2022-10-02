use crate::prelude::*;

pub enum SpawnTableType {
    Hostile,
    Item,
}

pub fn get_spawn_table_by_name(key: &str) -> MasterTable {
    let raw_master = RAWS.lock();
    if let Some(spawn_table) = raw_master.templates.spawn_tables.get(key) {
        let mut random_table = MasterTable::new();
        for e in spawn_table.iter() {
            random_table.add(e.name.clone(), e.weight, &raw_master);
        }
        random_table
    } else {
        panic!("Specified a spawn table name that doesnt exist");
    }
}

pub fn spawn_type_by_name(raws: &RawMaster, key: &str) -> SpawnTableType {
    if raws.hostile_index.contains_key(key) {
        SpawnTableType::Hostile
    } else {
        SpawnTableType::Item
    }
}

pub fn create_stats_from_raw_stats(raw_stats: &RawStats) -> Stats {
    let mut stats = Stats::default();
    if let Some(hp) = raw_stats.max_hp {
        stats.hp.max = hp;
        stats.hp.current = hp;
    }
    stats.base_armor = match raw_stats.base_armor {
        Some(base_armor) => base_armor,
        None => stats.base_armor,
    };
    stats.magic_resistance = match raw_stats.magic_resistance {
        Some(magic_resistance) => magic_resistance,
        None => stats.magic_resistance,
    };
    stats.physical_resistance = match raw_stats.physical_resistance {
        Some(physical_resistance) => physical_resistance,
        None => stats.physical_resistance,
    };
    stats.movement_cost = match raw_stats.movement_cost {
        Some(movement_cost) => movement_cost,
        None => stats.movement_cost,
    };
    stats.unarmed_damage = match raw_stats.unarmed_damage {
        Some(unarmed_damage) => unarmed_damage,
        None => stats.unarmed_damage,
    };
    stats.unarmed_attack_cost = match raw_stats.unarmed_attack_cost {
        Some(unarmed_attack_cost) => unarmed_attack_cost,
        None => stats.unarmed_attack_cost,
    };
    stats.chance_to_hit = match raw_stats.chance_to_hit {
        Some(chance_to_hit) => chance_to_hit,
        None => stats.chance_to_hit,
    };
    stats.chance_to_evade = match raw_stats.chance_to_evade {
        Some(chance_to_evade) => chance_to_evade,
        None => stats.chance_to_evade,
    };
    stats.chance_to_crit = match raw_stats.chance_to_crit {
        Some(chance_to_crit) => chance_to_crit,
        None => stats.chance_to_crit,
    };
    stats.crit_dmg_modifier = match raw_stats.crit_dmg_modifier {
        Some(crit_dmg_modifier) => crit_dmg_modifier,
        None => stats.crit_dmg_modifier,
    };

    stats
}

pub fn get_renderable_component(glyph: &RawGlyph) -> crate::ecs::Glyph {
    let fg = RGB::from(glyph.fg);
    let bg = RGB::from(glyph.bg.unwrap_or(BLACK));

    crate::ecs::Glyph {
        render_order: glyph.order,
        glyph: to_cp437(glyph.char),
        color: ColorPair::new(fg, bg),
    }
}

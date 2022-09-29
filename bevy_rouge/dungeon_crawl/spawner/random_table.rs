use super::*;

#[derive(Debug)]
pub struct RandomEntry {
    name: String,
    weight: i32,
}

impl RandomEntry {
    pub fn new<S: ToString>(name: S, weight: i32) -> RandomEntry {
        RandomEntry { name: name.to_string(), weight }
    }
}

#[derive(Default, Debug)]
pub struct MasterTable {
    pub items: RandomTable,
    pub mobs: RandomTable,
    pub props: RandomTable,
}

impl MasterTable {
    pub fn new() -> MasterTable {
        MasterTable { items: RandomTable::new(), mobs: RandomTable::new(), props: RandomTable::new() }
    }

    pub fn add<S: ToString>(&mut self, name: S, weight: i32, master: &TemplateMaster) {
        match master.spawn_type_by_name(&name.to_string()) {
            SpawnTableType::Mob => self.mobs.add(name, weight),
            SpawnTableType::Item => self.items.add(name, weight),
            SpawnTableType::Prop => self.props.add(name, weight),
        }
    }

    pub fn roll(&self, rng: &mut RandomNumbers) -> Option<String> {
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => self.items.roll(rng),
            _ => self.mobs.roll(rng),
            // _ => None,
        }
    }
}

#[derive(Default, Debug)]
pub struct RandomTable {
    total_weight: i32,
    entries: Vec<RandomEntry>,
}

impl RandomTable {
    pub fn new() -> RandomTable {
        RandomTable { entries: Vec::new(), total_weight: 0 }
    }

    pub fn add<S: ToString>(&mut self, name: S, weight: i32) {
        if weight > 0 {
            self.total_weight += weight;
            self.entries.push(RandomEntry::new(name.to_string(), weight));
        }
    }

    pub fn roll(&self, rng: &mut RandomNumbers) -> Option<String> {
        if self.total_weight == 0 {
            return None;
        }

        let mut roll = rng.roll_dice(1, self.total_weight) - 1;
        let mut index: usize = 0;

        while roll > 0 {
            if roll < self.entries[index].weight {
                return Some(self.entries[index].name.clone());
            }

            roll -= self.entries[index].weight;
            index += 1;
        }

        None
    }
}

use crate::dungeon_crawl::*;
use bevy::ecs::system::EntityCommands;
use std::collections::HashMap;

pub enum SpawnTableType {
    Item,
    Mob,
    Prop,
}

pub enum SpawnType {
    // Carried(Entity),
    // Equipped(Entity),
    AtPosition(Point),
}

impl TemplateMaster {
    pub fn spawn_type_by_name(&self, key: &str) -> SpawnTableType {
        if self.item_index.contains_key(key) {
            SpawnTableType::Item
        } else if self.mob_index.contains_key(key) {
            SpawnTableType::Mob
        } else {
            SpawnTableType::Prop
        }
    }

    pub fn spawn_position(&self, pos: SpawnType, eb: &mut EntityCommands, _tag: &str) {
        // Spawn in the specified location
        match pos {
            SpawnType::AtPosition(pt) => {
                eb.insert(pt);
            } // SpawnType::Carried(by) => {
              //     eb.insert(InBackpack { owner: by });
              // }
              // SpawnType::Equipped(by) => {
              //     let slot = find_slot_for_equippable_item(tag, raws);
              //     eb.insert(Equipped::new(by, slot));
              // }
        }
    }

    pub fn spawn_named_entity(
        &self,
        commands: &mut Commands,
        textures: &Res<TextureAssets>,
        spawn_key: &str,
        spawn_pt: SpawnType,
    ) -> Option<Entity> {
        if self.item_index.contains_key(spawn_key) {
            return self.spawn_named_item(commands, textures, spawn_key, spawn_pt);
        } else if self.mob_index.contains_key(spawn_key) {
            return self.spawn_named_mob(commands, textures, spawn_key, spawn_pt);
        }

        None
    }

    fn spawn_base_entity<T: BaseRawComponent + Clone>(
        &self,
        eb: &mut EntityCommands,
        entity_list: &[T],
        indexes: &HashMap<String, usize>,
        key: &str,
        pos: SpawnType,
    ) -> T {
        let entity_template = &entity_list[indexes[key]];
        self.spawn_position(pos, eb, key);

        eb.insert(TileSize::square(1.0));

        // Name Component
        eb.insert(Naming(entity_template.name()));

        // Description Component
        if let Some(desc) = entity_template.description() {
            eb.insert(Description(desc));
        }

        entity_template.clone()
    }

    fn spawn_named_mob(
        &self,
        commands: &mut Commands,
        textures: &Res<TextureAssets>,
        key: &str,
        pos: SpawnType,
    ) -> Option<Entity> {
        let mut entity = commands.spawn();
        let mob_template =
            self.spawn_base_entity(&mut entity, &self.templates.mobs, &self.mob_index, key, pos);

        let glyph = mob_template.glyph().clone();
        let (_color, zbuf) = (Color::hex(glyph.color.clone()).unwrap(), glyph.order);

        let (glyph, atlas) = match glyph.glyph {
            raws::templates::Glyph::Char(char) => (char as usize, textures.terminal.clone()),
            raws::templates::Glyph::Index(idx) => (idx, textures.undead.clone()),
        };

        entity.insert_bundle(SpriteSheetBundle {
            texture_atlas: atlas,
            visibility: Visibility { is_visible: false },
            sprite: get_sprite(glyph),
            transform: Transform::from_translation(Vec3::Z * zbuf as u16 as f32),
            ..Default::default()
        });

        let hp = mob_template.hp.unwrap();
        entity
            .insert(Health { current: hp, max: hp })
            .insert(ChasingPlayer)
            .insert(FieldOfView::new(mob_template.vision_range))
            .insert(Enemy);

        if mob_template.blocks_tile {
            entity.insert(BlocksTile);
        }

        Some(entity.id())
    }

    fn spawn_named_item(
        &self,
        commands: &mut Commands,
        textures: &Res<TextureAssets>,
        key: &str,
        pos: SpawnType,
    ) -> Option<Entity> {
        let mut entity = commands.spawn();
        let item_template =
            self.spawn_base_entity(&mut entity, &self.templates.items, &self.item_index, key, pos);

        let glyph = item_template.glyph().clone();
        let (color, zbuf) = (Color::hex(glyph.color.clone()).unwrap(), glyph.order);

        let (glyph, atlas) = match glyph.glyph {
            raws::templates::Glyph::Char(char) => (char as usize, textures.terminal.clone()),
            raws::templates::Glyph::Index(idx) => (idx, textures.potions.clone()),
        };

        entity.insert_bundle(SpriteSheetBundle {
            texture_atlas: atlas,
            visibility: Visibility { is_visible: false },
            sprite: get_sprite_with_color(glyph, color),
            transform: Transform::from_translation(Vec3::Z * zbuf as u16 as f32),
            ..Default::default()
        });

        entity.insert(Item);

        if let Some(effects) = &item_template.provides {
            effects.iter().for_each(|(provides, n)| match provides.as_str() {
                "Healing" => {
                    entity.insert(ProvidesHealing { amount: *n });
                }
                "MagicMap" => {
                    entity.insert(ProvidesDungeonMap);
                }
                _ => {
                    println!("Warning: we don't know how to provide {}", provides);
                }
            })
        }

        if let Some(damage) = item_template.base_damage {
            entity.insert(Damage(damage));
            entity.insert(Weapon);
        }

        Some(entity.id())
    }
}

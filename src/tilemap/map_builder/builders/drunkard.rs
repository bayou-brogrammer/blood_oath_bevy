use super::*;

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum DrunkSpawnMode {
    StartingPoint,
    Random,
}

pub struct DrunkardSettings {
    pub spawn_mode: DrunkSpawnMode,
    pub drunken_lifetime: i32,
    pub floor_percent: f32,
    pub brush_size: i32,
    pub symmetry: Symmetry,
}

pub struct DrunkardsWalkBuilder {
    settings: DrunkardSettings,
}

impl InitialMapArchitect for DrunkardsWalkBuilder {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        self.build(builder, rng);
    }
}

impl MapArchitect for DrunkardsWalkBuilder {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        self.build(builder, rng);
    }
}

impl DrunkardsWalkBuilder {
    #[allow(dead_code)]
    pub fn new(settings: DrunkardSettings) -> DrunkardsWalkBuilder {
        DrunkardsWalkBuilder { settings }
    }

    pub fn open_area() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::StartingPoint,
                drunken_lifetime: 400,
                floor_percent: 0.5,
                brush_size: 1,
                symmetry: Symmetry::None,
            },
        })
    }

    pub fn open_halls() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 400,
                floor_percent: 0.5,
                brush_size: 1,
                symmetry: Symmetry::None,
            },
        })
    }

    pub fn winding_passages() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4,
                brush_size: 1,
                symmetry: Symmetry::None,
            },
        })
    }

    pub fn fat_passages() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4,
                brush_size: 2,
                symmetry: Symmetry::None,
            },
        })
    }

    pub fn fearful_symmetry() -> Box<DrunkardsWalkBuilder> {
        Box::new(DrunkardsWalkBuilder {
            settings: DrunkardSettings {
                spawn_mode: DrunkSpawnMode::Random,
                drunken_lifetime: 100,
                floor_percent: 0.4,
                brush_size: 1,
                symmetry: Symmetry::Both,
            },
        })
    }

    fn build(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        // Set a central starting point
        let starting_position =
            Point { x: builder.map.width() / 2, y: builder.map.height() / 2 };
        let start_idx = builder.map.xy_idx(starting_position.x, starting_position.y);
        *builder.map.tiles.get_index_checked_mut(start_idx) = TileType::Floor;

        let total_tiles = builder.map.width() * builder.map.height();
        let desired_floor_tiles = (self.settings.floor_percent * total_tiles as f32) as usize;
        let mut floor_tile_count =
            builder.map.tiles.iter().filter(|a| **a == TileType::Floor).count();
        let mut digger_count = 0;
        while floor_tile_count < desired_floor_tiles {
            let mut drunk_x;
            let mut drunk_y;
            match self.settings.spawn_mode {
                DrunkSpawnMode::StartingPoint => {
                    drunk_x = starting_position.x;
                    drunk_y = starting_position.y;
                }
                DrunkSpawnMode::Random => {
                    if digger_count == 0 {
                        drunk_x = starting_position.x;
                        drunk_y = starting_position.y;
                    } else {
                        drunk_x = rng.roll_dice(1, builder.map.width() - 3) + 1;
                        drunk_y = rng.roll_dice(1, builder.map.height() - 3) + 1;
                    }
                }
            }
            let mut drunk_life = self.settings.drunken_lifetime;

            while drunk_life > 0 {
                let drunk_idx = builder.map.xy_idx(drunk_x, drunk_y);
                builder.paint(
                    self.settings.symmetry,
                    self.settings.brush_size,
                    drunk_x,
                    drunk_y,
                );
                *builder.map.tiles.get_index_checked_mut(drunk_idx) = TileType::DownStairs;

                let stagger_direction = rng.roll_dice(1, 4);
                match stagger_direction {
                    1 => {
                        if drunk_x > 2 {
                            drunk_x -= 1;
                        }
                    }
                    2 => {
                        if drunk_x < builder.map.width() - 2 {
                            drunk_x += 1;
                        }
                    }
                    3 => {
                        if drunk_y > 2 {
                            drunk_y -= 1;
                        }
                    }
                    _ => {
                        if drunk_y < builder.map.height() - 2 {
                            drunk_y += 1;
                        }
                    }
                }

                drunk_life -= 1;
            }

            digger_count += 1;
            for t in builder.map.tiles.iter_mut() {
                if *t == TileType::DownStairs {
                    *t = TileType::Floor;
                }
            }
            floor_tile_count =
                builder.map.tiles.iter().filter(|a| **a == TileType::Floor).count();
        }
    }
}

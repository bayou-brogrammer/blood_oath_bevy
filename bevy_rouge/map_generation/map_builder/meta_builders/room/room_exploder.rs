use super::*;

pub struct RoomExploder {}

impl MapArchitect for RoomExploder {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        self.build(builder, rng);
    }
}

impl RoomExploder {
    pub fn new() -> Box<RoomExploder> {
        Box::new(RoomExploder {})
    }

    fn build(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        let rooms: Vec<Rect>;
        if let Some(rooms_builder) = &builder.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Room Explosions require a builder with room structures");
        }

        for room in rooms.iter() {
            let start = room.center();
            let n_diggers = rng.roll_dice(1, 20) - 5;
            if n_diggers > 0 {
                for _i in 0..n_diggers {
                    let mut drunk_x = start.x;
                    let mut drunk_y = start.y;

                    let mut drunk_life = 20;
                    while drunk_life > 0 {
                        let drunk_idx = builder.map.xy_idx(drunk_x, drunk_y);

                        builder.paint(Symmetry::None, 1, drunk_x, drunk_y);
                        builder.map.tiles[drunk_idx] = TileType::DownStairs;

                        let stagger_direction = rng.roll_dice(1, 4);
                        match stagger_direction {
                            1 => {
                                if drunk_x > 2 {
                                    drunk_x -= 1;
                                }
                            }
                            2 => {
                                if drunk_x < builder.map.width - 2 {
                                    drunk_x += 1;
                                }
                            }
                            3 => {
                                if drunk_y > 2 {
                                    drunk_y -= 1;
                                }
                            }
                            _ => {
                                if drunk_y < builder.map.height - 2 {
                                    drunk_y += 1;
                                }
                            }
                        }

                        drunk_life -= 1;
                    }

                    for t in builder.map.tiles.iter_mut() {
                        if *t == TileType::DownStairs {
                            *t = TileType::Floor;
                        }
                    }
                }
            }
        }
    }
}

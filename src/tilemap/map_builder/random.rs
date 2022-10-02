use super::*;

impl BuilderChain {
    pub fn random_start_position(&self, rng: &RandomNumbers) -> (XStart, YStart) {
        let xroll = rng.roll_dice(1, 3);
        let x = match xroll {
            1 => XStart::Left,
            2 => XStart::Center,
            _ => XStart::Right,
        };

        let yroll = rng.roll_dice(1, 3);
        let y = match yroll {
            1 => YStart::Bottom,
            2 => YStart::Center,
            _ => YStart::Top,
        };

        (x, y)
    }

    pub fn random_shape_builder(&mut self, rng: &RandomNumbers) {
        let builder_roll = rng.roll_dice(1, 16);
        match builder_roll {
            1 => self.start_with(CellularAutomataArchitect::new()),
            2 => self.start_with(DrunkardsWalkBuilder::open_area()),
            3 => self.start_with(DrunkardsWalkBuilder::open_halls()),
            4 => self.start_with(DrunkardsWalkBuilder::winding_passages()),
            5 => self.start_with(DrunkardsWalkBuilder::fat_passages()),
            6 => self.start_with(DrunkardsWalkBuilder::fearful_symmetry()),
            _ => self.start_with(PrefabBuilder::constant(prefab_levels::WFC_POPULATED)),
        };
    }

    // pub fn random_room_builder(
    //     &mut self,
    //     master: &mut ResMut<TemplateMaster>,
    //     rng: &RandomNumbers,
    // ) {
    //     let build_roll = rng.roll_dice(1, 3);
    //     match build_roll {
    //         1 => self.start_with(RoomMapArchitect::new()),
    //         2 => self.start_with(BspDungeonBuilder::new()),
    //         _ => self.start_with(BspInteriorBuilder::new()),
    //     }

    //     println!("Random Room start with: {}", build_roll);

    //     // BSP Interior still makes holes in the walls
    //     // Sort by one of the 5 available algorithms
    //     let sort_roll = rng.roll_dice(1, 5);
    //     match sort_roll {
    //         1 => self.with(RoomSorter::new(RoomSort::Leftmost)),
    //         2 => self.with(RoomSorter::new(RoomSort::Rightmost)),
    //         3 => self.with(RoomSorter::new(RoomSort::Topmost)),
    //         4 => self.with(RoomSorter::new(RoomSort::Bottommost)),
    //         _ => self.with(RoomSorter::new(RoomSort::Central)),
    //     }

    //     self.with(RoomDrawer::new());

    //     let corridor_roll = rng.roll_dice(1, 4);
    //     match corridor_roll {
    //         1 => self.with(DoglegCorridors::new()),
    //         2 => self.with(NearestCorridors::new()),
    //         3 => self.with(StraightLineCorridors::new()),
    //         _ => self.with(BspCorridors::new()),
    //     }

    //     let cspawn_roll = rng.roll_dice(1, 2);
    //     if cspawn_roll == 1 {
    //         self.with(CorridorSpawner::new(master));
    //     }

    //     let modifier_roll = rng.roll_dice(1, 6);
    //     match modifier_roll {
    //         1 => self.with(RoomExploder::new()),
    //         2 => self.with(RoomCornerRounder::new()),
    //         _ => {}
    //     }

    //     let start_roll = rng.roll_dice(1, 2);
    //     match start_roll {
    //         1 => self.with(RoomBasedStartingPosition::new()),
    //         _ => {
    //             let (start_x, start_y) = self.random_start_position(rng);
    //             self.with(AreaStartingPosition::new(start_x, start_y));
    //         }
    //     }

    //     let exit_roll = rng.roll_dice(1, 2);
    //     match exit_roll {
    //         1 => self.with(RoomBasedStairs::new()),
    //         _ => self.with(DistantExit::new()),
    //     }

    //     let spawn_roll = rng.roll_dice(1, 2);
    //     match spawn_roll {
    //         1 => self.with(RoomBasedSpawner::new(master)),
    //         _ => self.with(VoronoiSpawning::new(master)),
    //     }
    // }
}

// pub fn random_builder(
//     new_depth: i32,
//     width: i32,
//     height: i32,
//     master: &mut ResMut<TemplateMaster>,
//     rng: &RandomNumbers,
// ) -> BuilderChain {
//     let mut builder = BuilderChain::new(new_depth, width, height, "New TileMap");

//     let type_roll = rng.roll_dice(1, 2);
//     match type_roll {
//         1 => builder.random_room_builder(master, rng),
//         _ => builder.random_shape_builder(rng),
//     }

//     builder.map_builder.map.theme = match new_depth {
//         0 => DungeonTheme::build(),
//         1 => ForestTheme::build(),
//         _ => CaveTheme::build(),
//     };

//     if rng.roll_dice(1, 3) == 1 {
//         builder.with(WaveformCollapseBuilder::new());

//         // Now set the start to a random starting area
//         let (start_x, start_y) = builder.random_start_position(rng);
//         builder.with(AreaStartingPosition::new(start_x, start_y));

//         // Setup an exit and spawn mobs
//         builder.with(VoronoiSpawning::new(master));
//         builder.with(DistantExit::new());
//     }

//     if rng.roll_dice(1, 20) == 1 {
//         builder.with(PrefabBuilder::sectional(prefab_sections::UNDERGROUND_FORT));
//     }

//     builder.with(WallBoundaries::new());
//     builder.with(PrefabBuilder::vaults());

//     builder
// }

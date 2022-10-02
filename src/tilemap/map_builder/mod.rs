use crate::prelude::*;

mod builders;
mod common;
mod meta_builders;
mod random;
// mod themes;

pub use builders::*;
pub use common::*;
pub use meta_builders::*;
pub use random::*;
// pub use themes::*;

pub type Idx = usize;

////////////////////////////////////////////////////////////////////////////////
// TileMap Builder Traits
////////////////////////////////////////////////////////////////////////////////

pub trait InitialMapArchitect {
    fn build_map(&mut self, map_builder: &mut MapBuilder, rng: &RandomNumbers);
}

pub trait MapArchitect {
    fn build_map(&mut self, map_builder: &mut MapBuilder, rng: &RandomNumbers);
}

pub trait MapTheme: std::fmt::Debug + Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> Option<Glyph>;
}

////////////////////////////////////////////////////////////////////////////////

pub struct BuilderChain {
    pub map_builder: MapBuilder,
    builders: Vec<Box<dyn MapArchitect>>,
    starter: Option<Box<dyn InitialMapArchitect>>,
}

impl BuilderChain {
    pub fn new<S: ToString>(size: grid_2d::Size, depth: i32, name: S) -> BuilderChain {
        BuilderChain {
            starter: None,
            builders: Vec::new(),
            map_builder: MapBuilder::new(size, depth, name),
        }
    }

    pub fn start_with(&mut self, starter: Box<dyn InitialMapArchitect>) -> &mut BuilderChain {
        match self.starter {
            None => self.starter = Some(starter),
            Some(_) => panic!("You can only have one starting builder."),
        };
        self
    }

    pub fn with(&mut self, metabuilder: Box<dyn MapArchitect>) -> &mut BuilderChain {
        self.builders.push(metabuilder);
        self
    }

    pub fn build_map(&mut self, rng: &RandomNumbers) {
        match &mut self.starter {
            None => panic!("Cannot run a map builder chain without a starting build system"),
            Some(starter) => {
                // Build the starting map
                starter.build_map(&mut self.map_builder, rng);
            }
        }

        // Build additional layers in turn
        for metabuilder in self.builders.iter_mut() {
            metabuilder.build_map(&mut self.map_builder, rng);
        }
    }
}

#[derive(Debug, Clone)]
pub struct MapBuilder {
    pub width: i32,
    pub height: i32,
    pub map: TileMap,
    pub rooms: Option<Vec<Rect>>,
    pub spawn_list: Vec<(Idx, String)>,
    pub corridors: Option<Vec<Vec<Idx>>>,
    pub starting_position: Option<Coord>,
}

impl MapBuilder {
    pub fn new<S: ToString>(size: grid_2d::Size, new_depth: i32, name: S) -> Self {
        Self {
            width: size.width() as i32,
            height: size.height() as i32,
            rooms: None,
            corridors: None,
            spawn_list: Vec::new(),
            starting_position: None,
            map: TileMap::new(size, new_depth, name),
        }
    }

    pub fn print_map(&self) {
        for (y, row) in self.map.tiles.rows().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let coord = Coord::new(x as i32, y as i32);
                let is_player = if let Some(player_coord) = self.starting_position {
                    coord == player_coord
                } else {
                    false
                };

                let ch = if is_player {
                    '@'
                } else {
                    match cell {
                        TileType::DownStairs => '>',
                        TileType::UpStairs => '<',
                        TileType::Floor => '.',
                        TileType::Wall => '#',
                        TileType::Door => '+',
                    }
                };
                print!("{}", ch);
            }
            println!();
        }
        println!();
    }
}

fn setup(mut commands: Commands, rng: Res<RandomNumbers>) {
    let mut builder = BuilderChain::new(grid_2d::Size::new(80, 50), 0, "New Map");
    builder
        .start_with(CellularAutomataArchitect::new_with_floor_percent(65))
        .with(RoomMapArchitect::new())
        .with(BspCorridors::new())
        .with(RoomDrawer::new())
        .with(CorridorSpawner::new())
        .with(SurroundWithWall::new())
        .with(AreaStartingPosition::new(XStart::Center, YStart::Center))
        .with(VoronoiSpawning::new("Default"))
        .with(CullUnreachable::new())
        .build_map(&rng);

    println!("Map built with {:?} rooms", builder.map_builder.map.size);

    commands.insert_resource(builder.map_builder.map.clone());
    commands.insert_resource(builder.map_builder);
    commands.insert_resource(NextState(GameState::InGame));
}

pub struct MapBuilderPlugin;
impl Plugin for MapBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Setup, setup);
    }
}

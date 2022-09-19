mod ecs;
mod loading;
mod rng;
mod spawner;
mod tilemap;
mod util;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_asset_loader::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use iyes_loopless::prelude::*;

    pub use bitvec::prelude::*;
    pub use serde::{Deserialize, Serialize};

    pub use bracket_geometry::prelude::*;
    pub use bracket_noise::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use bracket_random::prelude::*;
    pub use bracket_rex::prelude::*;

    pub use crate::ecs::*;
    pub use crate::loading::*;
    pub use crate::spawner::*;
    pub use crate::tilemap::*;
    pub use crate::util::*;

    pub const LAUNCHER_TITLE: &str = "Blood Oath";
    pub const WINDOW_WIDTH: f32 = 1280.0;
    pub const WINDOW_HEIGHT: f32 = 720.0;

    pub const VIEWPORT_WIDTH: f32 = 800.0;
    pub const VIEWPORT_HEIGHT: f32 = 600.0;
    pub const VIEWPORT_OFFSET: (f32, f32) = (0.0, 48.0);

    pub const DEBUG_MAP: bool = true;
    pub const TILEMAP_Z: f32 = 0.0;
    pub const MOB_Z: f32 = 3.0;
}

use bevy::render::texture::ImageSettings;
pub use prelude::*;

pub fn app() -> App {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: String::from("Tiled Map Editor Example"),
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .insert_resource(ImageSettings::default_nearest());

    // Game States Setup
    app.add_loopless_state(GameState::Loading);
    app.add_loopless_state(InGameState::AwaitingInput);

    app.add_plugins(DefaultPlugins)
        .add_plugin(LoadingPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(SpawnerPlugin)
        .add_plugin(EcsPlugin);

    app
}

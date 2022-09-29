#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod camera;
mod dungeon_crawl;
mod loading;
mod map;
mod menus;
mod random;
mod state;
mod utils;
mod world_gen;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::render::texture::ImageSettings;
    pub use bevy::winit::WinitSettings;
    pub use iyes_loopless::prelude::*;

    pub use bracket_geometry::prelude::*;
    pub use bracket_noise::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use bracket_random::prelude::*;
    pub use bracket_rex::prelude::*;

    pub use crate::GameElement;
    pub use crate::{impl_default, impl_new, switch_app_state};
    pub use sark_grids::prelude::*;

    pub use crate::camera::*;
    pub use crate::dungeon_crawl::*;
    pub use crate::loading::*;
    pub use crate::map::*;
    pub use crate::menus::*;
    pub use crate::random::*;
    pub use crate::state::*;
    pub use crate::utils::*;
    pub use crate::world_gen::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 60;
    pub const UI_HEIGHT: i32 = 10;
}

use bevy::render::texture::ImageSettings;
use bevy_ecs_tilemap::TilemapPlugin;
pub use prelude::*;

pub const LAUNCHER_TITLE: &str = "Bevy-RRogue";

#[derive(Component)]
pub struct GameElement;

pub fn app() -> App {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        fit_canvas_to_parent: true,
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        width: SCREEN_WIDTH as f32 * 10.0,
        height: SCREEN_HEIGHT as f32 * 10.0,
        ..Default::default()
    })
    .insert_resource(ImageSettings::default_nearest())
    .insert_resource(ClearColor(Color::hex("171717").unwrap()));

    app.add_loopless_state(AppState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(CameraPlugin)
        .add_plugins(MenuPlugins)
        .add_plugin(WorldGenPlugin)
        // .add_plugins(WorldGenerationPlugins)
        .add_plugin(DungeonCrawlPlugin);

    app
}

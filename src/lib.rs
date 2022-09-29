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
    pub use crate::rng::*;
    pub use crate::spawner::*;
    pub use crate::switch_in_game_state;
    pub use crate::tilemap::*;
    pub use crate::util::*;

    pub const LAUNCHER_TITLE: &str = "Blood Oath";
    pub const WINDOW_WIDTH: f32 = 1280.0;
    pub const WINDOW_HEIGHT: f32 = 720.0;

    pub const VIEWPORT_WIDTH: f32 = 50.0;
    pub const VIEWPORT_HEIGHT: f32 = 50.0;
    pub const VIEWPORT_OFFSET: (f32, f32) = (0.0, 0.0);

    pub const DEBUG_MAP: bool = true;
    pub const TILEMAP_Z: f32 = 0.0;
    pub const MOB_Z: f32 = 3.0;
}

use bevy::render::texture::ImageSettings;
use bevy_inspector_egui::{widgets::*, *};
pub use prelude::*;

#[derive(Inspectable, Default)]
struct Data {
    player: InspectorQuerySingle<&'static mut Position, With<Player>>,
}

pub fn app() -> Option<App> {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: String::from("Tiled Map Editor Example"),
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .insert_resource(ImageSettings::default_nearest());

    if cfg!(debug_assertions) {
        app.insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::INFO,
            filter: "gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,bevy_render::render_resource::pipeline_cache=debug".to_string(),
        });
    } else {
        app.insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::WARN,
            ..Default::default()
        });
    }

    app.add_plugins_with(DefaultPlugins, |group| {
        #[cfg(feature = "bundled")]
        group.add_before::<bevy::asset::AssetPlugin, _>(
            bevy_embedded_assets::EmbeddedAssetPlugin,
        );
        group
    });

    if cfg!(debug_assertions) {
        // app.add_plugin(::bevy::diagnostic::FrameTimeDiagnosticsPlugin).add_plugin(
        //     ::bevy::diagnostic::LogDiagnosticsPlugin::filtered(vec![
        //         ::bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS,
        //     ]),
        // );

        app.add_plugin(InspectorPlugin::<Data>::new()).register_inspectable::<Position>();
    }

    // Game States Setup
    app.add_loopless_state(GameState::Loading);
    app.add_loopless_state(InGameState::WhosTurn);

    app.add_plugin(LoadingPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(SpawnerPlugin)
        .add_plugin(EcsPlugin);

    #[cfg(feature = "debug-graph")]
    {
        bevy_mod_debugdump::print_schedule(&mut app);
        return None;
    }

    Some(app)
}

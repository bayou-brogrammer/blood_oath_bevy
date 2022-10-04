#[allow(clippy::type_complexity)]
mod ecs;
mod raws;
mod spawner;
mod tilemap;
mod util;

mod prelude {
    pub use bevy::prelude::*;
    pub use iyes_loopless::prelude::*;

    pub use bitvec::prelude::*;
    pub use serde::{Deserialize, Serialize};

    pub use bracket_bevy::prelude::*;
    pub use bracket_noise::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use direction::*;
    pub use grid_2d::*;

    pub use crate::ecs::*;
    pub use crate::raws::*;
    pub use crate::spawner::*;
    pub use crate::tilemap::*;
    pub use crate::util::*;
    pub use crate::{impl_default, impl_new, switch_in_game_state};

    pub const LAUNCHER_TITLE: &str = "Blood Oath";
    pub const WINDOW_WIDTH: f32 = 960.0;
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
    camera: InspectorQuerySingle<&'static mut Transform, With<CameraFollow>>,
}

pub fn app() -> Option<App> {
    let mut app = App::new();

    raws::load_raws();

    app.insert_resource(WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: String::from("Game"),
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
    app.add_loopless_state(GameState::Setup);
    app.add_loopless_state(InGameState::WhosTurn);

    app.add_plugin(
        BTermBuilder::empty()
            .with_random_number_generator(true)
            .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
            .with_font("terminal16x16.png", 16, 16, (18.0, 18.0))
            .with_font("dungeonfont.png", 16, 16, (32.0, 32.0))
            .with_font("vga8x16.png", 16, 16, (8.0, 16.0))
            .with_simple_console(2, 80, 50)
            .with_background(true), // .with_scaling_mode(TerminalScalingMode::ResizeTerminals),
    )
    .add_plugin(MapBuilderPlugin)
    .add_plugin(SpawnerPlugin)
    .add_plugin(EcsPlugin);

    #[cfg(feature = "debug-graph")]
    {
        bevy_mod_debugdump::print_schedule(&mut app);
        return None;
    }

    Some(app)
}

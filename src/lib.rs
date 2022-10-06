mod camera;
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
    pub use lazy_static::lazy_static;
    pub use serde::{Deserialize, Serialize};

    pub use bracket_bevy::prelude::*;
    pub use bracket_bevy::FontCharType;
    pub use bracket_noise::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use direction::*;
    pub use grid_2d::*;

    pub use crate::camera::*;
    pub use crate::ecs::*;
    pub use crate::raws::*;
    pub use crate::spawner::*;
    pub use crate::tilemap::*;
    pub use crate::util::*;
    pub use crate::{impl_default, impl_new, switch_in_game_state};

    pub const LAUNCHER_TITLE: &str = "Blood Oath";
    pub const WINDOW_WIDTH: f32 = 960.0;
    pub const WINDOW_HEIGHT: f32 = 720.0;

    // Screens
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;

    // Batches
    pub const BATCH_ZERO: usize = 0;
    pub const BATCH_CHARS: usize = 1000;
    // pub const BATCH_PARTICLES: usize = 2000;
    // pub const BATCH_UI: usize = 10_000;
    // pub const BATCH_UI_INV: usize = 15_000;
    // pub const BATCH_TOOLTIPS: usize = 100_000; // Over everything

    // Layers
    pub const LAYER_ZERO: usize = 0;
    pub const LAYER_CHAR: usize = 1;
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
            .with_scaling_mode(TerminalScalingMode::ResizeTerminals)
            .with_font("terminal8x8.png", 16, 16, (8.0, 8.0))
            .with_font("dungeonfont.png", 16, 16, (32.0, 32.0))
            .with_font("vga8x16.png", 16, 16, (8.0, 16.0))
            .with_simple_console(1, 80, 50)
            .with_background(true)
            .with_sparse_console(1, 80, 50)
            .with_background(false),
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

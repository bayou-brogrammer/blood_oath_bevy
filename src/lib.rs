mod ecs;
mod loading;
mod raws;
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

    pub use bracket_bevy::prelude::*;
    pub use direction::*;
    pub use grid_2d::*;
    // pub use bracket_geometry::prelude::*;
    pub use bracket_noise::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    // pub use bracket_random::prelude::*;
    // pub use bracket_rex::prelude::*;

    pub use crate::ecs::*;
    pub use crate::loading::*;
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
        title: String::from("Tiled TileMap Editor Example"),
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
            .with_font("terminal8x8.png", 16, 16, (16.0, 16.0))
            .with_font("vga8x16.png", 16, 16, (8.0, 16.0))
            .with_simple_console(0, 80, 50)
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

    app.add_system(tick.run_in_state(GameState::InGame));

    Some(app)
}

pub fn tick(map: Res<TileMap>, ctx: Res<BracketContext>, q: Query<(&Glyph, &Position)>) {
    for (idx, tile) in map.tiles.iter().enumerate() {
        let (glyph, color) = match tile {
            TileType::Floor => (to_cp437('.'), RGB::from_u8(127, 127, 127)),
            TileType::Wall => (to_cp437('#'), RGB::from_u8(125, 82, 44)),
            TileType::Door => (to_cp437('+'), RGB::from_u8(127, 127, 127)),
            _ => todo!(),
        };

        let coord = map.index_to_coord(idx);
        ctx.set(coord.x, coord.y, color, RGB::from_f32(0., 0., 0.), glyph);
    }

    for (g, p) in &q {
        ctx.set(p.x, p.y, g.color.fg, g.color.bg, g.glyph);
    }
}

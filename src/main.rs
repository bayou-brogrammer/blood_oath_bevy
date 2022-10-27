#![allow(clippy::type_complexity)]

pub mod debug;

mod bterm;
mod camera;
mod ecs;
mod noise;
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

    pub use crate::bterm::*;
    pub use crate::camera::*;
    pub use crate::ecs::*;
    pub use crate::noise::*;
    pub use crate::raws::*;
    pub use crate::spawner::*;
    pub use crate::tilemap::*;
    pub use crate::util::*;
    pub use crate::{impl_default, impl_new, switch_in_game_state, switch_turn_state};

    pub const LAUNCHER_TITLE: &str = "Blood Oath";
    pub const WINDOW_WIDTH: f32 = 960.0;
    pub const WINDOW_HEIGHT: f32 = 720.0;

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
pub use prelude::*;

pub fn main() {
    raws::load_raws();

    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: String::from("Game"),
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .insert_resource(ImageSettings::default_nearest());

    app.add_plugins_with(DefaultPlugins, |group| {
        #[cfg(feature = "bundled")]
        group.add_before::<bevy::asset::AssetPlugin, _>(
            bevy_embedded_assets::EmbeddedAssetPlugin,
        );
        group
    });

    // Game States Setup
    app.add_loopless_state(GameState::Setup);
    app.insert_resource(TurnState::AwaitingInput);

    // app.add_plugin(bterm::BTermPlugin)
    //     .add_plugin(tilemap::MapBuilderPlugin)
    //     .add_plugin(spawner::SpawnerPlugin)
    //     .add_plugin(ecs::EcsPlugin)
    //     .add_plugin(debug::DebugPlugin);

    app.add_plugin(bterm::BTermPlugin).add_plugin(debug::DebugPlugin);

    #[cfg(feature = "debug-graph")]
    {
        bevy_mod_debugdump::print_schedule(&mut app);
        return;
    }

    app.add_startup_system(setup).add_plugin(CameraPlugin).add_system(render_noise);

    app.run();
}

pub fn setup(mut commands: Commands, rng: Res<RandomNumbers>) {
    let mut nm = NoiseMap::default();
    nm.generate_maps();
    commands.insert_resource(nm);
}

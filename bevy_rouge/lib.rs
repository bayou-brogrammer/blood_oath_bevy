#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

mod camera;
mod dungeon_crawl;
mod loading;
mod map;
mod map_generation;
mod resources;
mod rng;
mod ui;
mod utils;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy::winit::WinitSettings;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use iyes_loopless::prelude::*;

    pub use bracket_geometry::prelude::*;
    pub use bracket_noise::prelude::*;
    pub use bracket_pathfinding::prelude::*;
    pub use bracket_random::prelude::*;
    pub use bracket_rex::prelude::*;

    pub use crate::camera::*;
    pub use crate::dungeon_crawl::*;
    pub use crate::loading::*;
    pub use crate::map::*;
    pub use crate::map_generation::*;
    pub use crate::resources::*;
    pub use crate::rng::*;
    pub use crate::ui::*;
    pub use crate::utils::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 60;
    pub const UI_HEIGHT: i32 = 10;
}

pub use bevy::render::texture::ImageSettings;
use prelude::*;

#[cfg(debug_assertions)]
struct DebugPlugin;
#[cfg(debug_assertions)]
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default());
    }
}

pub const LAUNCHER_TITLE: &str = "Bevy-RRogue";

#[derive(Component)]
pub struct GameElement;

pub fn app() -> App {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Roguelike Game".to_string(),
        width: SCREEN_WIDTH as f32 * 10.0,
        height: SCREEN_HEIGHT as f32 * 10.0,
        ..Default::default()
    })
    // Power-saving reactive rendering for applications.
    // .insert_resource(WinitSettings::desktop_app())
    .insert_resource(ImageSettings::default_nearest())
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

    app.add_state(AppState::Loading);
    app.add_loopless_state(AppState::Loading);
    app.insert_resource(TurnState::AwaitingInput);

    use crate::GameStage::*;
    app.add_stage_after(CoreStage::PostUpdate, Camera, SystemStage::single_threaded());
    app.add_stage_after(Camera, RenderPostUpdate, SystemStage::single_threaded());

    app.add_stage_after(CoreStage::Update, PlayerCombat, SystemStage::parallel())
        .add_stage_after(PlayerCombat, MovePlayer, SystemStage::parallel())
        .add_stage_after(MovePlayer, PlayerFov, SystemStage::parallel())
        .add_stage_after(PlayerFov, GenerateMonsterMoves, SystemStage::parallel())
        .add_stage_after(GenerateMonsterMoves, MonsterCombat, SystemStage::parallel())
        .add_stage_after(MonsterCombat, MoveMonsters, SystemStage::parallel())
        .add_stage_after(MoveMonsters, MonsterFov, SystemStage::parallel());

    app.add_plugins(DefaultPlugins)
        .add_plugin(LoadingPlugin)
        .add_plugin(ConsolePlugin)
        .add_plugin(TemplatePlugin)
        .add_plugin(MapGenPlugin)
        .add_plugin(SpawnerPlugin)
        .add_plugin(SystemsPlugin)
        .add_plugin(UIPlugin);

    // app.add_system_set_to_stage(
    //     GameStage::RenderPostUpdate,
    //     SystemSet::new().with_system(position_translation).with_system(size_scaling),
    // );

    #[cfg(feature = "debug")]
    app.add_plugin(DebugPlugin);

    app
}

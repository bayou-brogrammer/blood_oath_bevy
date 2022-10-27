use crate::prelude::*;

mod fov;
mod inventory;
mod map_indexing;
pub mod movement;
mod player;
mod render;

use fov::*;
use inventory::*;
use map_indexing::*;
use movement::*;
use player::*;
use render::*;

pub struct StartupIngameSystems;
impl Plugin for StartupIngameSystems {
    fn build(&self, app: &mut App) {
        // Startup Systems
        app.add_enter_system_set(
            GameState::InGame,
            SystemSet::new().with_system(map_indexing).with_system(fov.after(map_indexing)),
        );
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StartupIngameSystems)
            .add_plugin(PlayerPlugin)
            .add_plugin(RenderingPlugin)
            .add_plugin(InventoryPlugin);

        app.add_system_set_to_stage(
            CoreStage::Last,
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(movement)
                .with_system(fov)
                .into(),
        );

        app.add_system(
            switch_turn_state!(TurnState::AwaitingInput)
                .run_if_resource_equals(TurnState::PlayerTurn),
        );
    }
}

use crate::prelude::*;

mod fov;
mod map_indexing;
mod movement;
mod player;
mod whos_turn;

use fov::*;
use map_indexing::*;
use movement::*;
use player::*;
use whos_turn::*;

pub struct InGamePlugin;
impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        // InGame && WhosTurn
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_in_state(InGameState::WhosTurn)
                .with_system(whos_turn)
                .into(),
        );

        // InGame && AwaitingInput
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_in_state(InGameState::AwaitingInput)
                .with_system(player_input)
                .into(),
        );

        // InGame && ScoreAIActions
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_in_state(InGameState::ScoreAIActions)
                .with_system(switch_in_game_state!(InGameState::GenerateAIActions))
                .into(),
        );
        // add_ai_scoring_systems(app);

        // InGame && GenerateAIActions
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_in_state(InGameState::GenerateAIActions)
                .with_system(switch_in_game_state!(InGameState::ResolveActions))
                .into(),
        );
        // add_ai_action_generating_systems(app);

        // InGame && ResolveActions
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_in_state(InGameState::ResolveActions)
                .with_system(movement)
                .with_system(fov)
                // .with_system(unarmed_combat)
                .with_system(switch_in_game_state!(InGameState::WhosTurn))
                .into(),
        );

        // After ResolveActions
        app.add_exit_system_set(
            InGameState::ResolveActions,
            SystemSet::new()
                // .with_system(cull_dead)
                .with_system(map_indexing)
                .with_system(fov.after(map_indexing)), // .with_system(map_indexing.after(cull_dead)),
        );
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InGamePlugin);

        // # InGame #
        app.add_enter_system_set(
            GameState::InGame,
            SystemSet::new().with_system(map_indexing).with_system(fov.after(map_indexing)),
        );

        app.add_system(fov.run_in_state(GameState::InGame));
    }
}

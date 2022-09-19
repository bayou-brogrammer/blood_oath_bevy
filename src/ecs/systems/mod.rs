use crate::prelude::*;

mod camera;
mod movement;
mod player;

pub use camera::*;
pub use movement::*;
pub use player::*;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin);

        // // InGame && WhosTurn
        // app.add_system_set(
        //     ConditionSet::new()
        //         .run_in_state(GameState::InGame)
        //         .run_in_state(InGameState::WhosTurn)
        //         .with_system(whos_turn)
        //         .into(),
        // );

        // InGame && AwaitingInput
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_in_state(InGameState::AwaitingInput)
                .with_system(player_input)
                .into(),
        );

        // InGame && ResolveActions
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_in_state(InGameState::AwaitingInput)
                .with_system(movement)
                // .with_system(unarmed_combat)
                // .with_system(switch_in_game_state!(InGameState::WhosTurn))
                .into(),
        );
    }
}

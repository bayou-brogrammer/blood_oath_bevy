use crate::dungeon_crawl::*;

mod camera;
mod chasing;
mod combat;
mod end_turn;
mod fov;
mod map_indexing;
mod movement;
mod player_input;
mod update_entities_visibility;
mod use_items;

// struct AwaitingInputPlugin;
// impl Plugin for AwaitingInputPlugin {
//     fn build(&self, app: &mut App) {
//         // Startup Systems
//         app.add_enter_system(AppState::Playing, camera::camera_move);

//         app.add_plugin(player_input::PlayerInputPlugin);

//         app.add_system_set(
//             ConditionSet::new()
//                 .run_in_state(AppState::Playing)
//                 .run_if_resource_equals(TurnState::TurnState::AwaitingInput)
//                 .with_system(fov::fov)
//                 .with_system(update_entities_visibility::update_entities_visibility)
//                 .into(),
//         );
//     }
// }

// struct PlayerPlugin;
// impl Plugin for PlayerPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_system_set_to_stage(
//             PlayerStage::GenerateActions,
//             ConditionSet::new()
//                 .run_in_state(AppState::Playing)
//                 .run_if_resource_equals(TurnState::TurnState::PlayerTurn)
//                 .with_system(movement::movement)
//                 .with_system(combat::combat)
//                 .with_system(use_items::use_items)
//                 .into(),
//         );

//         app.add_system_set_to_stage(
//             PlayerStage::HandleActions,
//             ConditionSet::new()
//                 .run_in_state(AppState::Playing)
//                 .run_if_resource_equals(TurnState::TurnState::PlayerTurn)
//                 // .with_system(camera::camera_move)
//                 .with_system(map_indexing::map_indexing)
//                 .into(),
//         );

//         // CLeanup
//         app.add_system_set_to_stage(
//             PlayerStage::Cleanup,
//             ConditionSet::new()
//                 .run_in_state(AppState::Playing)
//                 .run_if_resource_equals(TurnState::TurnState::PlayerTurn)
//                 .with_system(fov::fov)
//                 .with_system(end_turn::end_turn)
//                 .into(),
//         );
//     }
// }

// struct MonsterPlugin;
// impl Plugin for MonsterPlugin {
//     fn build(&self, app: &mut App) {
//         // Generate Actions
//         app.add_system_set_to_stage(
//             AIStage::HandleAI,
//             ConditionSet::new()
//                 .run_in_state(AppState::Playing)
//                 .run_if_resource_equals(TurnState::TurnState::MonsterTurn)
//                 .with_system(chasing::chasing)
//                 .into(),
//         );

//         // Generate Actions
//         app.add_system_set_to_stage(
//             AIStage::GenerateActions,
//             ConditionSet::new()
//                 .run_in_state(AppState::Playing)
//                 .run_if_resource_equals(TurnState::TurnState::MonsterTurn)
//                 .with_system(combat::combat)
//                 .with_system(movement::movement)
//                 .into(),
//         );

//         app.add_system_set_to_stage(
//             AIStage::HandleActions,
//             ConditionSet::new()
//                 .run_if_resource_equals(TurnState::TurnState::MonsterTurn)
//                 .run_in_state(AppState::Playing)
//                 .with_system(map_indexing::map_indexing)
//                 .with_system(fov::fov)
//                 .into(),
//         );

//         app.add_system_set_to_stage(
//             AIStage::Cleanup,
//             ConditionSet::new()
//                 .run_in_state(AppState::Playing)
//                 .run_if_resource_equals(TurnState::TurnState::MonsterTurn)
//                 .with_system(end_turn::end_turn)
//                 .into(),
//         );
//     }
// }

// pub struct SystemsPlugin;
// impl Plugin for SystemsPlugin {
//     fn build(&self, app: &mut App) {
//         app.add_plugin(AwaitingInputPlugin).add_plugin(PlayerPlugin).add_plugin(MonsterPlugin);
//     }
// }
use crate::GameStage::*;

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Playing)
                .label(SystemLabels::Fov)
                .with_system(fov::fov)
                .with_system(update_entities_visibility::update_entities_visibility)
                .into(),
        );

        app.add_system_set_to_stage(GameStage::Camera, SystemSet::new().with_system(camera::camera_move));

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::Playing)
                .run_if_resource_equals(TurnState::AwaitingInput)
                .with_system(player_input::player_input.chain(player_input::player_result))
                .into(),
        );

        app.add_system_set_to_stage(
            PlayerCombat,
            ConditionSet::new()
                .run_in_state(AppState::Playing)
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(use_items::use_items)
                .with_system(combat::combat)
                .into(),
        );

        app.add_system_set_to_stage(
            MovePlayer,
            ConditionSet::new()
                .label("move_player")
                .before("camera_move")
                .run_in_state(AppState::Playing)
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(movement::movement)
                .into(),
        );

        app.add_system_set_to_stage(
            PlayerFov,
            ConditionSet::new()
                .run_in_state(AppState::Playing)
                .run_if_resource_equals(TurnState::PlayerTurn)
                .with_system(fov::fov)
                .with_system(end_turn::end_turn)
                .into(),
        );

        app.add_system_set_to_stage(
            GenerateMonsterMoves,
            ConditionSet::new()
                .run_in_state(AppState::Playing)
                .run_if_resource_equals(TurnState::MonsterTurn)
                // .with_system(random_move::random_move)
                .with_system(chasing::chasing)
                .into(),
        );

        app.add_system_set_to_stage(
            MonsterCombat,
            ConditionSet::new()
                .run_in_state(AppState::Playing)
                .run_if_resource_equals(TurnState::MonsterTurn)
                .with_system(use_items::use_items)
                .with_system(combat::combat)
                .into(),
        );

        app.add_system_set_to_stage(
            MoveMonsters,
            ConditionSet::new()
                .run_in_state(AppState::Playing)
                .run_if_resource_equals(TurnState::MonsterTurn)
                .with_system(movement::movement)
                .into(),
        );

        app.add_system_set_to_stage(
            MonsterFov,
            ConditionSet::new()
                .run_in_state(AppState::Playing)
                .run_if_resource_equals(TurnState::MonsterTurn)
                .with_system(fov::fov)
                .with_system(end_turn::end_turn)
                .into(),
        );
    }
}

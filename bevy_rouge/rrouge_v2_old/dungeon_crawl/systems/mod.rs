use crate::dungeon_crawl::*;

mod chasing;
mod initiative;
mod movement;
mod player_input;
mod world_update;

use chasing::chasing;
use initiative::initiative;
use movement::movement;
use player_input::{player_input, player_result};
use world_update::update_world_map;

pub struct SystemPlugin;
impl Plugin for SystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::DungeonCrawl(TurnState::AwaitingInput))
                .with_system(player_input.chain(player_result))
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .label(SystemLabels::MoveEntity)
                .run_in_state(AppState::DungeonCrawl(TurnState::Ticking))
                .with_system(movement)
                .with_system(initiative)
                .with_system(update_world_map)
                .with_system(chasing)
                .into(),
        );
    }
}

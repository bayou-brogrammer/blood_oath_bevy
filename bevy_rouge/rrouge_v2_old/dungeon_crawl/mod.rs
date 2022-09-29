use crate::prelude::*;

pub mod spawner;

mod components;
mod console;
mod events;
mod systems;

pub use components::*;
pub use console::*;
pub use events::*;
pub use systems::*;

pub struct DungeonCrawlPlugin;
impl Plugin for DungeonCrawlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EventPlugin).add_plugin(SystemPlugin);
        app.init_resource::<Console>();

        app.add_system(
            switch_app_state!(AppState::DungeonCrawl(TurnState::AwaitingInput))
                .run_in_state(AppState::DungeonCrawlEnter),
        );
    }
}

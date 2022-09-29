use crate::prelude::*;

mod components;
mod console;
mod raws;
mod renderutils;
mod spawner;
mod systems;

pub use components::*;
pub use console::*;
pub use raws::*;
pub use renderutils::*;
pub use spawner::*;
pub use systems::*;

struct DungeonCrawlPlugin;
impl Plugin for DungeonCrawlPlugin {
    fn build(&self, app: &mut App) {}
}

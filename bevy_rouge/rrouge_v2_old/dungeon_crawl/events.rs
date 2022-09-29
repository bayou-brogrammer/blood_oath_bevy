use crate::dungeon_crawl::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WantsToMove(pub Entity, pub Point, pub Point);

pub struct EventPlugin;
impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<WantsToMove>();
    }
}

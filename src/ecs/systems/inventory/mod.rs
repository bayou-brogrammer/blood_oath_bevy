use crate::prelude::*;

mod collection;
pub use collection::*;

pub struct InventoryPlugin;
impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        // Inventory Events
        app.add_system_set_to_stage(
            CoreStage::Update,
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_on_event::<WantsToPickupItem>()
                .with_system(item_collection)
                .into(),
        );
    }
}

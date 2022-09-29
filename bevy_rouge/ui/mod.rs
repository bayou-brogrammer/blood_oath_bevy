use crate::prelude::*;

mod equipment;
mod hud;
mod inventory;
mod popup;
mod splash_screen;
mod tooltips;

#[derive(Component)]
pub struct TopUINode;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(splash_screen::MenuPlugin);
        // .add_plugin(hud::HudPlugin)
        // .add_plugin(popup::PopUpPlugin)
        // .add_plugin(tooltips::TooltipsPlugin);
    }
}

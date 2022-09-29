use crate::prelude::*;
use bevy::app::PluginGroupBuilder;

mod main_menu;

pub struct MenuPlugins;
impl PluginGroup for MenuPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(main_menu::MainMenuPlugin);
    }
}

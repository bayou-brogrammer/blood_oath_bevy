use crate::prelude::*;

mod area;
mod random_table;

pub use area::*;
pub use random_table::*;

pub fn spawn_player(mut commands: Commands, map_builder: Res<MapBuilder>) {
    let start_pos = &map_builder.starting_position.expect("No player start found");

    spawn_player_from_raw(&mut commands, *start_pos);
}

pub fn spawn_others(mut commands: Commands, map_builder: Res<MapBuilder>) {
    for (idx, name) in map_builder.spawn_list.iter() {
        if let Some(starting_position) = map_builder.starting_position {
            let coord = map_builder.map.index_to_coord(*idx);
            if coord != starting_position
                && spawn_named_entity(&mut commands, name, SpawnType::AtPosition(coord))
                    .is_none()
            {
                println!("Entity with name {} not found in raws", name);
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////

pub struct SpawnerPlugin;
impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system_set(
            GameState::Setup,
            SystemSet::new().with_system(spawn_player).with_system(spawn_others),
        );
    }
}

#[macro_export]
macro_rules! switch_in_game_state {
    ($e:expr) => {
        |mut commands: Commands| {
            commands.insert_resource(NextState($e));
        }
    };
}

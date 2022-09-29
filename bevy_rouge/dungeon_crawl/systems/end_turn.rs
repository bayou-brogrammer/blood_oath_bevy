use crate::dungeon_crawl::*;

pub fn end_turn(mut commands: Commands, turn_state: Res<TurnState>) {
    // calculate new turn
    let current_state = *turn_state;
    let new_state = match *turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => current_state,
    };

    commands.insert_resource(new_state);
}

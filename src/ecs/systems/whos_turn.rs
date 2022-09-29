use crate::prelude::*;

pub fn whos_turn(
    mut commands: Commands,
    existing_turns_q: Query<Option<&Player>, With<MyTurn>>,
    mut wait_q: Query<(Entity, &mut WaitingForTurn, Option<&Player>)>,
) {
    if wait_q.is_empty() && existing_turns_q.is_empty() {
        panic!("No entities with WaitingForTurn or MyTurn. Avoiding infinite loop.")
    }

    let mut keep_ticking = true;
    let mut player_turn = false;

    if existing_turns_q.is_empty() {
        while keep_ticking {
            for (entity, mut wait, player) in wait_q.iter_mut() {
                wait.ticks_until_turn -= 1;
                if wait.ticks_until_turn < 1 {
                    keep_ticking = false;
                    commands.entity(entity).remove::<WaitingForTurn>();
                    commands.entity(entity).insert(MyTurn { turn: wait.turns_taken + 1 });
                    if player.is_some() {
                        player_turn = true;
                    }
                }
            }
        }
    } else {
        // a turn action failed, so they get to try again
        if existing_turns_q.iter().any(|p| p.is_some()) {
            player_turn = true;
        }
    }

    if player_turn {
        // give player priority to take their turn
        commands.insert_resource(NextState(InGameState::AwaitingInput));
    } else {
        commands.insert_resource(NextState(InGameState::ScoreAIActions));
    }
}

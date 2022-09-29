use crate::dungeon_crawl::*;

pub fn initiative(
    mut commands: Commands,
    rng: Res<RandomNumbers>,
    state: Res<CurrentState<AppState>>,
    turns: Query<Entity, With<MyTurn>>,
    mut initiative_order: Query<(Entity, &mut Initiative, &Point, Option<&Player>)>,
) {
    if state.0 != AppState::DungeonCrawl(TurnState::Ticking) {
        return;
    }

    // Clear any remaining MyTurn we left by mistkae
    for e in &turns {
        commands.entity(e).remove::<MyTurn>();
    }

    for (entity, mut initiative, pos, player) in &mut initiative_order {
        initiative.current -= 1;

        if initiative.current < 1 {
            // It's my turn!
            commands.entity(entity).insert(MyTurn {});

            // Re-roll
            initiative.current = 6 + rng.roll_dice(1, 6);

            // Give a bonus for quickness
            // if let Some(attr) = attributes.get(entity) {
            //     initiative.current -= attr.quickness.bonus;
            // }

            // TODO: More initiative granting boosts/penalties will go here later

            // If its the player, we want to go to an AwaitingInput state
            if player.is_some() {
                commands.insert_resource(NextState(AppState::DungeonCrawl(TurnState::AwaitingInput)));
            }
        }
    }
}

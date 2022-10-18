#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Loading,
    Setup,
    InGame,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnState {
    WhosTurn,
    AwaitingInput,
    ResolveActions,
    ScoreAIActions,
    GenerateAIActions,
    Dead,
}

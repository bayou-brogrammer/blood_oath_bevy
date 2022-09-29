use bevy::prelude::{StageLabel, SystemLabel};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TurnState {
    // WorldUpdate,
    // Turn,
    AwaitingInput,
    Ticking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    GameOver,

    WorldGeneration,
    DungeonCrawlEnter,
    DungeonCrawl(TurnState),
    DungeonCrawlExitToMenu,
    // DungeonCrawlDescend,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, StageLabel)]
pub enum PlayerStage {
    GenerateActions,
    HandleActions,
    Cleanup,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, StageLabel)]
pub enum AIStage {
    HandleAI,
    GenerateActions,
    HandleActions,
    Cleanup,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, SystemLabel)]
pub enum SystemLabels {
    MoveEntity,
    CameraMove,
}

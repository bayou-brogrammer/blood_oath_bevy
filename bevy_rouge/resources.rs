use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum AppState {
    Loading,
    MainMenu,
    MapGen,
    Playing,
    NextLevel,
    GameOver,
    Victory,
}

// not a resource in bevy but hands on defines it as resource. We will use Bevy State
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum TurnState {
    // Menu States

    // Game Statess
    EquipmentPopup,
    InventoryPopup,

    // Actor States
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, SystemLabel)]
pub enum SystemLabels {
    SpawnEntities,
    Fov,
    MoveEntity,
    CameraMove,
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    // The first stage (player input) is the standard Update
    PlayerCombat,
    MovePlayer,
    PlayerFov,
    GenerateMonsterMoves,
    MonsterCombat,
    MoveMonsters,
    MonsterFov,
    Camera,
    RenderPostUpdate,
}

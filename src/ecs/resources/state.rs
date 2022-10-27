use crate::ReflectResource;
use bevy::reflect::Reflect;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    Setup,
    InGame,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TurnState {
    #[default]
    AwaitingInput,
    PlayerTurn,
    EnemyTurn,
    Dead,
}

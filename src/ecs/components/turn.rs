use crate::prelude::*;

#[derive(Component)]
pub struct WaitingForTurn {
    pub ticks_until_turn: u32,
    pub turns_taken: u32,
}

impl Default for WaitingForTurn {
    fn default() -> Self {
        Self { ticks_until_turn: 1, turns_taken: 0 }
    }
}

#[derive(Component)]
pub struct MyTurn {
    pub turn: u32,
}

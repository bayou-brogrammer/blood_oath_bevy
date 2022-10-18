use crate::prelude::*;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Component)]
pub struct Position(pub Coord);

impl Default for Position {
    fn default() -> Self {
        Self(Coord::new(0, 0))
    }
}

impl Deref for Position {
    type Target = Coord;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Coord> for Position {
    fn from(coord: Coord) -> Self {
        Position(coord)
    }
}

use std::ops::{Deref, DerefMut};

use crate::prelude::*;

#[derive(Debug, Component)]
pub struct Position(pub Point);

impl Default for Position {
    fn default() -> Self {
        Self(Point::zero())
    }
}

impl Deref for Position {
    type Target = Point;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Point> for Position {
    fn from(pt: Point) -> Self {
        Position(pt)
    }
}

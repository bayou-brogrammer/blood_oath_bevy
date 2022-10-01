use crate::prelude::*;

#[derive(Debug)]
/// WantsToMove uses Point because Point supports negative values
/// which is important for walking off a map on the W/S edges
pub struct WantsToMove(pub Entity, pub Coord);

#[derive(Debug)]
/// (Who is attacking, Who they are attacking)
pub struct WantsToAttack(pub Entity, pub Entity);

use crate::prelude::*;
use std::collections::HashSet;

#[derive(Bundle)]
pub struct RenderableBundle {
    pub name: Name,
    pub glyph: Glyph,
    pub position: Position,
}

impl RenderableBundle {
    pub fn new<S: ToString>(
        name: S,
        coord: Coord,
        glyph: FontCharType,
        color: ColorPair,
    ) -> Self {
        Self {
            position: Position(coord),
            name: Name::new(name.to_string()),
            glyph: Glyph::new(glyph, color),
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub tag: Player,
    pub fov: FieldOfView,
    pub stats: Stats,

    #[bundle]
    pub render: RenderableBundle,
}

impl PlayerBundle {
    pub fn new<S: ToString>(
        coord: Coord,
        name: S,
        glyph: FontCharType,
        color: ColorPair,
        stats: Stats,
    ) -> Self {
        Self {
            stats,
            tag: Player,
            fov: FieldOfView { visible_tiles: HashSet::new(), radius: 8 },
            render: RenderableBundle::new(name, coord, glyph, color),
        }
    }
}

#[derive(Bundle, Component)]
pub struct HostileBundle {
    pub tag: Hostile,
    pub fov: FieldOfView,
    pub stats: Stats,
    pub blocks_tile: BlocksMovement,

    #[bundle]
    pub render: RenderableBundle,
}

impl HostileBundle {
    pub fn new<S: ToString>(
        coord: Coord,
        name: S,
        glyph: FontCharType,
        color: ColorPair,
        stats: Stats,
    ) -> Self {
        Self {
            stats,
            tag: Hostile,
            fov: FieldOfView { visible_tiles: HashSet::new(), radius: 8 },
            blocks_tile: BlocksMovement,
            render: RenderableBundle::new(name, coord, glyph, color),
        }
    }
}

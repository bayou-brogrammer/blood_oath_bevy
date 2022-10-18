use crate::prelude::*;

#[derive(Bundle, Component)]
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
        order: RenderOrder,
    ) -> Self {
        Self {
            position: Position(coord),
            name: Name::new(name.to_string()),
            glyph: Glyph::new(glyph, color, order),
        }
    }
}

#[derive(Bundle, Component)]
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
            fov: FieldOfView::new(8),
            render: RenderableBundle::new(name, coord, glyph, color, RenderOrder::Actor),
        }
    }
}

#[derive(Bundle)]
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
            fov: FieldOfView::new(6),
            blocks_tile: BlocksMovement,
            render: RenderableBundle::new(name, coord, glyph, color, RenderOrder::Actor),
        }
    }
}

#[derive(Bundle, Component)]
pub struct ItemBundle {
    pub tag: Item,

    #[bundle]
    pub render: RenderableBundle,
}

impl ItemBundle {
    pub fn new<S: ToString>(
        coord: Coord,
        name: S,
        glyph: FontCharType,
        color: ColorPair,
    ) -> Self {
        Self {
            tag: Item,
            render: RenderableBundle::new(name, coord, glyph, color, RenderOrder::Actor),
        }
    }
}

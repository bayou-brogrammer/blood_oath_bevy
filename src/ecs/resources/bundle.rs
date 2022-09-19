use crate::prelude::*;
use std::collections::HashSet;

#[derive(Bundle, Component)]
pub struct PlayerBundle {
    pub tag: Player,
    pub fov: FieldOfView,

    #[bundle]
    pub render: RenderableBundle,
}

impl PlayerBundle {
    pub fn new(pos: Point, atlas: Handle<TextureAtlas>) -> Self {
        let idx = get_tile_index(GameSymbol::Player) as usize;
        Self {
            tag: Player,
            fov: FieldOfView { visible_tiles: HashSet::new(), radius: 10 },
            render: RenderableBundle::new("Player", pos, atlas, idx, true),
        }
    }
}

#[derive(Bundle, Component)]
pub struct RenderableBundle {
    pub name: Name,
    pub position: Position,

    #[bundle]
    pub sprite: SpriteSheetBundle,
}

impl RenderableBundle {
    pub fn new<S: ToString>(
        name: S,
        pos: Point,
        texture_atlas: Handle<TextureAtlas>,
        texture_index: usize,
        is_visible: bool,
    ) -> Self {
        let (x, y) = pt_spritecoords(pos);
        Self {
            name: Name::new(name.to_string()),
            position: Position(pos),
            sprite: SpriteSheetBundle {
                texture_atlas,
                visibility: Visibility { is_visible },
                transform: Transform::from_xyz(x, y, MOB_Z),
                sprite: get_sprite(texture_index),
                ..default()
            },
        }
    }
}

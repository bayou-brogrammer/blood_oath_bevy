use crate::prelude::*;
use std::collections::HashSet;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub tag: Player,
    pub fov: FieldOfView,
    pub light: LightSource,

    #[bundle]
    pub render: RenderableBundle,
}

impl PlayerBundle {
    pub fn new(pos: Point, atlas: Handle<TextureAtlas>) -> Self {
        let idx = get_tile_index(TileSets::Ascii, GameSymbol::Player) as usize;
        Self {
            tag: Player,
            light: LightSource { color: Color::WHITE, range: 8 },
            fov: FieldOfView { visible_tiles: HashSet::new(), radius: 8 },
            render: RenderableBundle::new("Player", pos, atlas, idx, true),
        }
    }
}

#[derive(Bundle)]
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

#[derive(Bundle)]
pub struct EnemyBundle {
    pub tag: Enemy,
    pub fov: FieldOfView,
    pub light: LightSource,
    pub blocks: BlocksMovement,

    #[bundle]
    pub render: RenderableBundle,
}

impl EnemyBundle {
    pub fn new(pt: Point, atlas: Handle<TextureAtlas>) -> Self {
        let idx = get_tile_index(TileSets::Ascii, GameSymbol::Enemy) as usize;
        Self {
            blocks: BlocksMovement,
            tag: Enemy,
            light: LightSource { color: Color::WHITE, range: 8 },
            fov: FieldOfView { visible_tiles: HashSet::new(), radius: 5 },
            render: RenderableBundle::new("Enemy", pt, atlas, idx, true),
        }
    }
}

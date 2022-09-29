use crate::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod builders;
pub use builders::*;

pub const MAP_SIZE: f32 = 60. * 8.;

fn setup_world(mut commands: Commands, rng: Res<RandomNumbers>, textures: Res<TextureAssets>) {
    let mut builder = builders::random_builder(60, 60, 1);
    builder.build_map(&rng);

    let tilemap_size = TilemapSize { x: 60, y: 60 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };

    let mut tile_storage = TileStorage::empty(tilemap_size);
    let tilemap_entity = commands.spawn().id();

    let map = builder.get_map();
    map.tiles.iter().enumerate().for_each(|(i, tile)| {
        let pt = map.index_to_point2d(i);
        let tile_pos = TilePos { x: pt.x as u32, y: pt.y as u32 };
        let texture = match tile {
            TileType::Floor => 71,
            TileType::Wall => 3,
            TileType::DownStairs => 13,
            TileType::UpStairs => 14,
        };

        let tile_entity = commands
            .spawn()
            .insert_bundle(TileBundle {
                texture: TileTexture(texture),
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                ..Default::default()
            })
            .id();
        tile_storage.set(&tile_pos, Some(tile_entity));
    });

    commands.entity(tilemap_entity).insert_bundle(TilemapBundle {
        tile_size,
        size: tilemap_size,
        storage: tile_storage,
        grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
        texture: TilemapTexture(textures.tileset.clone()),
        transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(&tilemap_size, &tile_size, 0.0),
        ..Default::default()
    });

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(16., 16.)),
                ..Default::default()
            },
            texture_atlas: textures.rogue.clone(),
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                ZBUF_PLAYER,
            ),
            ..Default::default()
        })
        .insert(builder.starting_pos())
        .insert(Player { map_level: 0, facing: Facing::Down })
        .insert(Name::new("Player".to_string()))
        .insert(Initiative { current: 0 });

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(16., 16.)),
                ..Default::default()
            },
            texture_atlas: textures.undead.clone(),
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                ZBUF_CREATURES,
            ),
            ..Default::default()
        })
        .insert(builder.starting_pos() + Point::new(2, 2))
        .insert(Health { current: 10, max: 20 })
        .insert(Enemy)
        .insert(Naming("AI".to_string()))
        .insert(FieldOfView::new(7))
        .insert(ChasingPlayer)
        .insert(Damage(1))
        .insert(Initiative { current: 2 });

    commands.insert_resource(builder.get_map());
    commands.insert_resource(NextState(AppState::DungeonCrawlEnter));
}

pub struct WorldGenPlugin;
impl Plugin for WorldGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::WorldGeneration, setup_world);
    }
}

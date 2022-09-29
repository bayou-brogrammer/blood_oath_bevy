use crate::prelude::*;

mod map_builder;
pub use map_builder::*;

fn generate_map(
    mut commands: Commands,
    player_q: Query<&Player>,
    images: Res<TextureAssets>,
    mut master: ResMut<TemplateMaster>,
    mut rng: ResMut<RandomNumbers>,
) {
    // start by getting the player, if it exists, to get the level
    // if it doesnt exist, then it is level 0
    let mut level = 0;
    if player_q.iter().count() > 0 {
        level = player_q.single().map_level;
        // increase level by 1, because this system gets executed before the post_nextlevel
        level += 1;
    }

    // create map
    println!("Creating map for level {}", level);
    let mb = MapBuilder::level_builder(level, 80, 50, &mut master, &mut rng);

    let tilemap_size = TilemapSize { x: 80, y: 50 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };

    let mut tile_storage = TileStorage::empty(tilemap_size);
    let tilemap_entity = commands.spawn().id();

    let map = mb.map.clone();
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
        texture: TilemapTexture(images.tileset.clone()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });

    // insert map builder as resource
    commands.insert_resource(mb);
    commands.insert_resource(map);
    commands.insert_resource(NextState(AppState::Playing));
}

pub struct MapGenPlugin;
impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::MapGen, generate_map);
    }
}

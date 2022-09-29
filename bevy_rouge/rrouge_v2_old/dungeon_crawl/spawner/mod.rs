use crate::dungeon_crawl::*;

pub fn spawn_player(commands: &mut Commands, textures: &TextureAssets, start_pos: Point) {
    // println!("Spawning player at {:?}", start_pos);

    // commands
    //     .spawn_bundle(SpriteSheetBundle {
    //         sprite: TextureAtlasSprite {
    //             index: 0,
    //             custom_size: Some(Vec2::new(16., 16.)),
    //             ..Default::default()
    //         },
    //         texture_atlas: textures.rogue.clone(),
    //         transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
    //             &tilemap_size,
    //             &tile_size,
    //             ZBUF_PLAYER,
    //         ),
    //         ..Default::default()
    //     })
    //     .insert(builder.starting_pos())
    //     .insert(Player { map_level: 0, facing: Facing::Down })
    //     .insert(Name::new("Player".to_string()))
    //     .insert(Initiative { current: 0 });
}

pub fn spawn_ai(commands: &mut Commands, textures: &TextureAssets, start_pos: Point) {
    println!("Spawning player at {:?}", start_pos);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: get_sprite(0),
            texture_atlas: textures.undead.clone(),
            transform: Transform::from_translation(Vec3::Z * ZBUF_CREATURES),
            ..Default::default()
        })
        .insert(start_pos)
        // .insert(TileSize::square(1.0))
        .insert(Health { current: 10, max: 20 })
        .insert(Enemy)
        .insert(Naming("AI".to_string()))
        .insert(FieldOfView::new(4))
        .insert(ChasingPlayer)
        .insert(Damage(1))
        .insert(Initiative { current: 2 });
}

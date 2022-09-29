use crate::dungeon_crawl::*;

pub fn update_entities_visibility(
    console: Res<Console>,
    player_fov_q: Query<&FieldOfView, With<Player>>,
    mut entities_q: Query<(
        Entity,
        &Point,
        &mut Visibility,
        Option<&MapTile>,
        Option<&mut Sprite>,
        Option<&mut TextureAtlasSprite>,
    )>,
    names_enemies_q: Query<&Naming, With<Enemy>>,
    names_items_q: Query<&Naming, With<Item>>,
) {
    // get the player fov
    let player_fov = player_fov_q.single();

    // for every etity with position
    for (ent, pos, mut vis, map_tile, sprite, atlas_sprite) in entities_q.iter_mut() {
        // first check if it is a map tile or some other entity. If it is a map tile...
        if map_tile.is_some() {
            if player_fov.visible_tiles.contains(pos) {
                // make it visible
                vis.is_visible = true;
                // increase the color alpha to 1, to both sprites or atlas_sprite
                if let Some(mut sprite) = sprite {
                    sprite.color.set_a(1.0);
                }
                if let Some(mut atlas_sprite) = atlas_sprite {
                    atlas_sprite.color.set_a(1.0);
                }
            } else if vis.is_visible {
                // if visible true but not in fov, tint
                // decrease the color alpha, to both sprites or atlas_sprite
                if let Some(mut sprite) = sprite {
                    sprite.color.set_a(0.1);
                }
                if let Some(mut atlas_sprite) = atlas_sprite {
                    atlas_sprite.color.set_a(0.1);
                }
            }
        } else {
            // if it is not a map tile, but some character or entity
            // if this thing is on the player fov, make it visible
            if player_fov.visible_tiles.contains(pos) {
                // if it was not visible before, make it appear and describe in gamelog
                if !vis.is_visible {
                    vis.is_visible = true;
                    // if enemy, get name update gamelog
                    if let Ok(name) = names_enemies_q.get(ent) {
                        console.write(Logger::new().append(name.0.clone()).append("appears.").to_str());
                    }
                    // if item, provide hint
                    if let Ok(name) = names_items_q.get(ent) {
                        console
                            .write(Logger::new().append("Press G to grab").append(name.0.clone()).to_str());
                    }
                }
            } else {
                // otherwise make it invisible
                vis.is_visible = false;
            }
        }
    }
}

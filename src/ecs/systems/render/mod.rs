use crate::prelude::*;

pub fn render_map(map: Res<TileMap>, camera: Res<CameraView>, ctx: Res<BracketContext>) {
    let mut batch = ctx.new_draw_batch();
    batch.target(LAYER_ZERO);
    batch.cls();

    camera.viewport.for_each(|pt| {
        let coord = pt.to_coord();
        let screen_pt = camera.world_to_screen(pt);
        if map.in_bounds(coord) && map.is_revealed(coord) {
            let (glyph, color) = map.tile_glyph(coord);
            batch.set(screen_pt, color, glyph);
        }
    });

    ctx.submit_batch(BATCH_ZERO, batch);
}

pub fn render_entities(
    map: Res<TileMap>,
    camera: Res<CameraView>,
    ctx: Res<BracketContext>,
    renderables: Query<(&Glyph, &Position)>,
) {
    let mut batch = ctx.new_draw_batch();
    batch.target(LAYER_CHAR);
    batch.cls();

    let mut entities = renderables.iter().collect::<Vec<_>>();
    entities.sort_by(|&a, &b| b.0.render_order.cmp(&a.0.render_order));
    for (glyph, pos) in entities {
        if map.is_visible(**pos) {
            let screen_pt = camera.world_to_screen(pos.to_point());
            batch.set(screen_pt, glyph.color, glyph.glyph);
        }
    }

    ctx.submit_batch(BATCH_CHARS, batch);
}

pub struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_map.run_in_state(GameState::InGame))
            .add_system(render_entities.run_in_state(GameState::InGame));
    }
}

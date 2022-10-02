use crate::prelude::*;

pub fn render_all(
    map: Res<TileMap>,
    ctx: Res<BracketContext>,
    renderables: Query<(&Glyph, &Position)>,
) {
    let mut batch = ctx.new_draw_batch();
    for (coord, _) in map.tiles.enumerate() {
        if map.is_revealed(coord) {
            let (glyph, color) = map.tile_glyph(coord);
            batch.set(coord.to_point(), color, glyph);
        }
    }

    for (g, p) in &renderables {
        if map.is_visible(**p) {
            batch.set(p.to_point(), g.color, g.glyph);
        }
    }

    ctx.submit_batch(1000, batch);
}

pub struct RenderingPlugin;
impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_all.run_in_state(GameState::InGame));
    }
}

use crate::prelude::*;

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/ascii_tilesets/ascii_plus.png")]
    pub ascii_tileset: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 32, rows = 31,))]
    #[asset(path = "textures/ascii_tilesets/ascii_plus.png")]
    pub ascii_tileset_atlas: Handle<TextureAtlas>,
}

#[derive(AssetCollection)]
pub struct MapAssets {}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<TextureAssets>()
                .with_collection::<MapAssets>()
                .continue_to_state(GameState::Setup),
        );
    }
}

use crate::prelude::*;

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/colored_transparent_packed.png")]
    pub tileset: Handle<Image>,

    #[asset(texture_atlas(
        tile_size_x = 16.,
        tile_size_y = 16.,
        columns = 48,
        rows = 22,
        // padding_x = 1.,
        // padding_y = 1.
    ))]
    #[asset(path = "textures/colored_transparent_packed.png")]
    pub tilset_atlas: Handle<TextureAtlas>,
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

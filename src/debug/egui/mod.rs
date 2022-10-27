use crate::prelude::*;
use bevy_egui::*;

pub fn noise_generator(
    rng: Res<RandomNumbers>,
    mut map: ResMut<NoiseMap>,
    mut egui: ResMut<EguiContext>,
) {
    let base_settings = map.settings.clone();

    egui::Window::new("Noising").show(egui.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut map.settings.octaves, 0..=20).text("Octaves"));
        ui.add(
            egui::Slider::new(&mut map.settings.persistence, 0.0..=20.0).text("Persistence"),
        );

        ui.add(
            egui::Slider::new(&mut map.settings.biome_map_frequency, 0.0..=1.0)
                .text("Biome Map Scale"),
        );
        ui.add(
            egui::Slider::new(&mut map.settings.height_map_frequency, 0.0..=1.0)
                .text("Height Map Scale"),
        );

        //////////////////////////////////////////////////////////////////////////////
        ui.add(
            egui::Slider::new(&mut map.settings.height_map_mult, 0.0..=5.0)
                .text("Height Map Multi"),
        );

        ui.add(
            egui::Slider::new(&mut map.settings.gradient_mult, 0.0..=5.0)
                .text("Gradient Multi"),
        );

        ui.add(
            egui::Slider::new(&mut map.settings.biome_map_sub, 0.0..=5.0)
                .text("Biome Map Sub"),
        );

        ui.add(
            egui::Slider::new(&mut map.settings.biome_map_mult, 0.0..=5.0)
                .text("Biome Map Multi"),
        );

        if ui.button("Reseed").clicked() {
            map.seed = rng.rand::<i64>();
            map.generate_maps();
        }
    });

    if base_settings != map.settings {
        map.generate_maps();
    }
}

pub struct EguiInspectorPlugin;
impl Plugin for EguiInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);

        app.add_system(noise_generator);
    }
}

use crate::prelude::*;
use bevy::diagnostic::{
    Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy_egui::*;

pub fn noise_generator(
    rng: Res<RandomNumbers>,
    mut map: ResMut<InternalNoiseMap>,
    mut egui: ResMut<EguiContext>,
) {
    let base_settings = map.settings.clone();

    egui::Window::new("Noising").show(egui.ctx_mut(), |ui| {
        ui.add(egui::Slider::new(&mut map.settings.octaves, 0..=20).text("Octaves"));
        ui.add(
            egui::Slider::new(&mut map.settings.persistence, 0.0..=20.0).text("Persistence"),
        );
        ui.add(egui::Slider::new(&mut map.settings.lacunarity, 0.0..=20.0).text("Lacunarity"));

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
            egui::Slider::new(&mut map.settings.height_map_gradient_mult, 0.0..=5.0)
                .text("Height Map Gradient Multi"),
        );

        ui.add(
            egui::Slider::new(&mut map.settings.biome_map_sub, 0.0..=5.0)
                .text("Biome Map Sub"),
        );
        ui.add(
            egui::Slider::new(&mut map.settings.biome_map_gradient_mult, 0.0..=5.0)
                .text("Biome Map Gradient Multi"),
        );

        ui.add(egui::Slider::new(&mut map.settings.low, 0.0..=10.0).text("Low"));
        ui.add(egui::Slider::new(&mut map.settings.high, 0.0..=10.0).text("High"));

        if ui.button("Reseed").clicked() {
            map.seed = rng.rand::<i64>();
            map.generate_maps();
        }
    });

    if base_settings != map.settings {
        map.generate_maps();
    }
}

/// This system will then change the title during execution
fn change_title(mut windows: ResMut<Windows>, diagnostics: Res<Diagnostics>) {
    if let Some(window) = windows.get_primary_mut() {
        let title = format!(
            "Avg. FPS: {:.02} | Entity Count: {}",
            diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .unwrap()
                .average()
                .unwrap_or_default(),
            diagnostics
                .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                .unwrap()
                .value()
                .unwrap_or_default()
        );

        window.set_title(title);
    }
}

pub struct EguiInspectorPlugin;
impl Plugin for EguiInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(EntityCountDiagnosticsPlugin)
            .add_stage_after(
                CoreStage::PostUpdate,
                "debug_ui_stage",
                SystemStage::parallel().with_system_set(
                    SystemSet::new().with_system(change_title).with_system(noise_generator),
                ),
            );
    }
}

use crate::prelude::*;
use bevy::diagnostic::{
    Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy_egui::*;

pub fn noise_generator(
    mut commands: Commands,
    rng: Res<RandomNumbers>,
    mut map: ResMut<InternalNoiseMap>,
    mut egui: ResMut<EguiContext>,
) {
    // let base_settings = map.settings.clone();

    let mut changed = false;

    egui::Window::new("Noising").show(egui.ctx_mut(), |ui| {
        changed |= ui
            .add(egui::Slider::new(&mut map.settings.octaves, 0..=20).text("Octaves"))
            .changed();

        changed |= ui
            .add(
                egui::Slider::new(&mut map.settings.persistence, 0.0..=20.0)
                    .text("Persistence"),
            )
            .changed();
        changed |= ui
            .add(
                egui::Slider::new(&mut map.settings.lacunarity, 0.0..=20.0).text("Lacunarity"),
            )
            .changed();

        //////////////////////////////////////////////////////////////////////////////
        // Height Map
        //////////////////////////////////////////////////////////////////////////////
        changed |= ui
            .add(
                egui::Slider::new(&mut map.settings.height_map_frequency, 0.0..=1.0)
                    .text("Height Map Scale"),
            )
            .changed();

        changed |= ui
            .add(
                egui::Slider::new(&mut map.settings.height_map_mult, 0.0..=5.0)
                    .text("Height Map Multi"),
            )
            .changed();

        changed |= ui
            .add(
                egui::Slider::new(&mut map.settings.height_map_gradient_mult, 0.0..=5.0)
                    .text("Height Map Gradient Multi"),
            )
            .changed();

        //////////////////////////////////////////////////////////////////////////////
        // Water Map
        //////////////////////////////////////////////////////////////////////////////
        changed |= ui
            .add(
                egui::Slider::new(&mut map.settings.moisture_map_frequency, 0.0..=1.0)
                    .text("Biome Map Scale"),
            )
            .changed();

        changed |= ui
            .add(
                egui::Slider::new(&mut map.settings.moisture_map_sub, 0.0..=5.0)
                    .text("Biome Map Sub"),
            )
            .changed();
        changed |= ui
            .add(
                egui::Slider::new(&mut map.settings.moisture_map_gradient_mult, 0.0..=5.0)
                    .text("Biome Map Gradient Multi"),
            )
            .changed();

        changed |=
            ui.add(egui::Slider::new(&mut map.settings.low, 0.0..=10.0).text("Low")).changed();
        changed |= ui
            .add(egui::Slider::new(&mut map.settings.high, 0.0..=10.0).text("High"))
            .changed();

        ui.horizontal(|ui| {
            ui.label(format!("Seed: {}", map.seed));

            if ui.button("Reseed").clicked() {
                map.seed = rng.next_u64();
                changed |= true;
            }
        });
    });

    if changed {
        commands.insert_resource(map.generate_maps());
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

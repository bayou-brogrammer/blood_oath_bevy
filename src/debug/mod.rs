use crate::prelude::*;
use bevy::{diagnostic::EntityCountDiagnosticsPlugin, prelude::App};

mod cli;
mod egui;

pub use cli::*;
pub use egui::*;

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        let cli = cli::parse_cli();

        if cfg!(debug_assertions) {
            app.insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::INFO,
            filter: "gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,bevy_render::render_resource::pipeline_cache=debug".to_string(),
          });

            // Fps / Entity Tracking
            app.add_plugin(::bevy::diagnostic::FrameTimeDiagnosticsPlugin)
                .add_plugin(EntityCountDiagnosticsPlugin);

            app.add_plugin(EguiInspectorPlugin);

            // FPS Text
            if cli.show_fps {
                app.add_plugin(::bevy::diagnostic::LogDiagnosticsPlugin::filtered(vec![
                    ::bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS,
                ]));
            }
        } else {
            app.insert_resource(bevy::log::LogSettings {
                level: bevy::log::Level::WARN,
                ..Default::default()
            });
        }
    }
}

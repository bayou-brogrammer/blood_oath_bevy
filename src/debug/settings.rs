use crate::prelude::*;
use bevy::{diagnostic::EntityCountDiagnosticsPlugin, prelude::App};
use bevy_inspector_egui::widgets::*;
use bevy_inspector_egui::*;

use super::cli;

#[derive(Inspectable, Default)]
struct InspectorData {
    player: InspectorQuerySingle<&'static mut Position, With<Player>>,
    // camera: InspectorQuerySingle<&'static mut Transform, With<BracketCamera>>,
}

pub fn debug_settings(app: &mut App) {
    let cli = cli::parse_cli();

    if cfg!(debug_assertions) {
        app.insert_resource(bevy::log::LogSettings {
          level: bevy::log::Level::INFO,
          filter: "gfx_backend_metal=warn,wgpu_core=warn,bevy_render=info,lain=debug,bevy_render::render_resource::pipeline_cache=debug".to_string(),
        });

        app.add_plugin(::bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugin(EntityCountDiagnosticsPlugin);

        // Inspector Plugin
        app.add_plugin(InspectorPlugin::<InspectorData>::new())
            .register_inspectable::<Position>();

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

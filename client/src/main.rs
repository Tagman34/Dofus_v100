mod game;
mod network;
mod ui;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Dofus-like Client".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .init_resource::<game::GameState>()
        .init_resource::<network::NetworkConnection>()
        .add_event::<network::NetworkEvent>()
        .add_systems(Startup, (setup_camera, game::setup_map))
        .add_systems(
            Update,
            (
                game::update_players,
                game::handle_input,
                network::handle_network_events,
                network::receive_from_server,
                ui::ui_system,
            ),
        )
        .run();
}

/// Système pour configurer la caméra
fn setup_camera(mut commands: Commands) {
    // Caméra isométrique
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::new(5.0, 0.0, 5.0), Vec3::Y),
        ..default()
    });

    // Lumière
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 3000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
        ..default()
    });
}

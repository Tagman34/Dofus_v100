use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::game::GameState;
use crate::network;

/// Ressource pour les paramètres de connexion
#[derive(Resource)]
pub struct ConnectionSettings {
    pub server_address: String,
    pub status_message: String,
    pub connecting: bool,
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        Self {
            server_address: "127.0.0.1:8080".to_string(),
            status_message: String::new(),
            connecting: false,
        }
    }
}

/// Système pour afficher le menu principal
pub fn main_menu_system(
    mut contexts: EguiContexts,
    mut connection_settings: ResMut<ConnectionSettings>,
    mut network_connection: ResMut<network::NetworkConnection>,
    mut next_state: ResMut<NextState<crate::AppState>>,
) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);
            
            ui.heading(
                egui::RichText::new("🎮 Dofus-like")
                    .size(48.0)
                    .color(egui::Color32::from_rgb(100, 200, 100)),
            );
            
            ui.add_space(20.0);
            ui.label(
                egui::RichText::new("Jeu Multijoueur Tour par Tour")
                    .size(20.0)
                    .color(egui::Color32::GRAY),
            );

            ui.add_space(60.0);

            // Cadre de connexion
            egui::Frame::none()
                .fill(egui::Color32::from_rgb(30, 30, 40))
                .rounding(10.0)
                .inner_margin(30.0)
                .show(ui, |ui| {
                    ui.set_min_width(400.0);
                    
                    ui.label(
                        egui::RichText::new("Connexion au serveur")
                            .size(24.0)
                            .color(egui::Color32::WHITE),
                    );
                    
                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        ui.label("Adresse du serveur:");
                        ui.text_edit_singleline(&mut connection_settings.server_address);
                    });

                    ui.add_space(10.0);

                    if !connection_settings.status_message.is_empty() {
                        let color = if connection_settings.connecting {
                            egui::Color32::YELLOW
                        } else if connection_settings.status_message.contains("Erreur") {
                            egui::Color32::RED
                        } else {
                            egui::Color32::GREEN
                        };
                        
                        ui.label(
                            egui::RichText::new(&connection_settings.status_message)
                                .color(color),
                        );
                    }

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        if ui
                            .add_sized(
                                [150.0, 40.0],
                                egui::Button::new(
                                    egui::RichText::new("Se connecter")
                                        .size(16.0)
                                        .color(egui::Color32::WHITE),
                                ),
                            )
                            .clicked()
                            && !connection_settings.connecting
                        {
                            // Lance la connexion
                            connection_settings.connecting = true;
                            connection_settings.status_message =
                                "Connexion en cours...".to_string();

                            let address = connection_settings.server_address.clone();
                            
                            match network::connect_to_server_blocking(&address) {
                                Ok(stream) => {
                                    network_connection.stream = Some(stream);
                                    network_connection.connected = true;
                                    connection_settings.status_message =
                                        "Connexion réussie !".to_string();
                                    connection_settings.connecting = false;
                                    
                                    // Passe à l'état de jeu
                                    next_state.set(crate::AppState::InGame);
                                }
                                Err(e) => {
                                    connection_settings.status_message =
                                        format!("Erreur: {}", e);
                                    connection_settings.connecting = false;
                                }
                            }
                        }

                        if ui
                            .add_sized(
                                [150.0, 40.0],
                                egui::Button::new(
                                    egui::RichText::new("Quitter")
                                        .size(16.0)
                                        .color(egui::Color32::WHITE),
                                ),
                            )
                            .clicked()
                        {
                            std::process::exit(0);
                        }
                    });
                });

            ui.add_space(40.0);

            ui.label(
                egui::RichText::new("Instructions:")
                    .size(16.0)
                    .color(egui::Color32::GRAY),
            );
            ui.label("• Flèches/WASD: Déplacer votre personnage");
            ui.label("• Espace: Terminer votre tour");
            ui.label("• Cliquez sur un joueur pour l'attaquer (à portée)");
        });
    });
}

/// Système pour afficher l'interface utilisateur pendant le jeu
pub fn ui_system(mut contexts: EguiContexts, game_state: Res<GameState>) {
    egui::Window::new("HUD")
        .title_bar(false)
        .resizable(false)
        .anchor(egui::Align2::LEFT_TOP, [10.0, 10.0])
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("État du jeu");

            if let Some(ref world_state) = game_state.world_state {
                ui.label(format!("Tour: {}", world_state.turn_number));
                ui.label(format!("Joueur actif: {}", world_state.current_turn));

                if let Some(my_id) = game_state.my_player_id {
                    if let Some(player) = world_state.get_player(my_id) {
                        ui.separator();
                        ui.label(format!("Votre joueur (ID: {})", my_id));
                        ui.label(format!(
                            "Position: ({}, {})",
                            player.position.x, player.position.y
                        ));
                        ui.label(format!("PA: {}", player.action_points));
                        ui.label(format!("PM: {}", player.movement_points));
                        ui.label(format!("Vie: {}/{}", player.health, player.max_health));
                        ui.label(format!(
                            "État: {}",
                            if player.is_alive { "Vivant" } else { "Mort" }
                        ));

                        if world_state.current_turn == my_id {
                            ui.label(
                                egui::RichText::new("C'est votre tour !")
                                    .color(egui::Color32::GREEN),
                            );
                        }
                    }
                }

                ui.separator();
                ui.label("Autres joueurs:");
                for player in &world_state.players {
                    if Some(player.id) != game_state.my_player_id {
                        ui.label(format!(
                            "Joueur {}: ({}, {}) - Vie: {}/{}",
                            player.id,
                            player.position.x,
                            player.position.y,
                            player.health,
                            player.max_health
                        ));
                    }
                }
            } else {
                ui.label("En attente de synchronisation...");
            }

            ui.separator();
            ui.label("Contrôles:");
            ui.label("Flèches/WASD: Déplacer");
            ui.label("Espace: Terminer le tour");
        });
}

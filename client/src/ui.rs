use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::game::GameState;

/// Système pour afficher l'interface utilisateur
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
                        ui.label(format!("Position: ({}, {})", player.position.x, player.position.y));
                        ui.label(format!("PA: {}", player.action_points));
                        ui.label(format!("PM: {}", player.movement_points));
                        ui.label(format!("Vie: {}/{}", player.health, player.max_health));
                        ui.label(format!("État: {}", if player.is_alive { "Vivant" } else { "Mort" }));

                        if world_state.current_turn == my_id {
                            ui.label(egui::RichText::new("C'est votre tour !").color(egui::Color32::GREEN));
                        }
                    }
                }

                ui.separator();
                ui.label("Autres joueurs:");
                for player in &world_state.players {
                    if Some(player.id) != game_state.my_player_id {
                        ui.label(format!(
                            "Joueur {}: ({}, {}) - Vie: {}/{}",
                            player.id, player.position.x, player.position.y, player.health, player.max_health
                        ));
                    }
                }
            } else {
                ui.label("En attente de connexion...");
            }

            ui.separator();
            ui.label("Contrôles:");
            ui.label("Flèches/WASD: Déplacer");
            ui.label("Espace: Terminer le tour");
        });
}


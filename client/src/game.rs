use bevy::prelude::*;
use bevy::mesh::primitives::Capsule3d;
use bevy::mesh::Plane3d;
use shared::protocol::{PlayerId, PlayerState, Position, WorldState};

/// Composant représentant un joueur sur la carte
#[derive(Component)]
pub struct Player {
    pub id: PlayerId,
}

/// Composant pour la position sur la grille
#[derive(Component)]
pub struct GridPosition {
    pub position: Position,
}

/// Ressource contenant l'état du monde
#[derive(Resource, Default)]
pub struct GameState {
    pub world_state: Option<WorldState>,
    pub my_player_id: Option<PlayerId>,
}

/// Marqueur pour la carte
#[derive(Component)]
pub struct MapTile;

/// Système pour mettre à jour l'affichage des joueurs
pub fn update_players(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    player_query: Query<(Entity, &Player, &mut Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Some(ref world_state) = game_state.world_state {
        // Supprime les joueurs qui n'existent plus
        let current_player_ids: Vec<PlayerId> = world_state.players.iter().map(|p| p.id).collect();
        for (entity, player, _) in player_query.iter() {
            if !current_player_ids.contains(&player.id) {
                commands.entity(entity).despawn();
            }
        }

        // Ajoute ou met à jour les joueurs
        for player_state in &world_state.players {
            let existing = player_query
                .iter()
                .find(|(_, p, _)| p.id == player_state.id);

            if let Some((entity, _, mut transform)) = existing {
                // Met à jour la position
                transform.translation.x = player_state.position.x as f32 * 1.0;
                transform.translation.z = player_state.position.y as f32 * 1.0;
                transform.translation.y = 0.5;
            } else {
                // Crée un nouveau joueur
                let color = if Some(player_state.id) == game_state.my_player_id {
                    Color::rgb(0.0, 1.0, 0.0) // Vert pour le joueur local
                } else {
                    Color::rgb(1.0, 0.0, 0.0) // Rouge pour les autres
                };

                let mesh_handle = meshes.add(Capsule3d::default());
                let material_handle = materials.add(StandardMaterial {
                    base_color: color,
                    ..default()
                });

                commands.spawn((
                    Player { id: player_state.id },
                    GridPosition {
                        position: player_state.position,
                    },
                    Mesh3d(mesh_handle),
                    MaterialMeshBundle {
                        mesh: mesh_handle.clone(),
                        material: material_handle,
                        transform: Transform::from_xyz(
                            player_state.position.x as f32,
                            0.5,
                            player_state.position.y as f32,
                        ),
                        ..default()
                    },
                ));
            }
        }
    }
}

/// Système pour créer la carte de base
pub fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Crée une grille 10x10
    for x in 0..10 {
        for y in 0..10 {
            let is_dark = (x + y) % 2 == 0;
            let color = if is_dark {
                Color::rgb(0.3, 0.5, 0.3)
            } else {
                Color::rgb(0.4, 0.6, 0.4)
            };

            let mesh_handle = meshes.add(Plane3d::default().mesh().size(1.0));
            let material_handle = materials.add(StandardMaterial {
                base_color: color,
                ..default()
            });

            commands.spawn((
                MapTile,
                Mesh3d(mesh_handle.clone()),
                MaterialMeshBundle {
                    mesh: mesh_handle,
                    material: material_handle,
                    transform: Transform::from_xyz(x as f32, 0.0, y as f32),
                    ..default()
                },
            ));
        }
    }
}

/// Système pour gérer les entrées clavier
pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: Res<GameState>,
    mut network_events: EventWriter<crate::network::NetworkEvent>,
) {
    if let Some(my_id) = game_state.my_player_id {
        let mut moved = false;
        let mut target_position = None;

        if keyboard_input.just_pressed(KeyCode::ArrowUp) || keyboard_input.just_pressed(KeyCode::KeyW) {
            if let Some(ref world_state) = game_state.world_state {
                if let Some(player) = world_state.get_player(my_id) {
                    target_position = Some(Position::new(player.position.x, player.position.y - 1));
                    moved = true;
                }
            }
        }

        if keyboard_input.just_pressed(KeyCode::ArrowDown) || keyboard_input.just_pressed(KeyCode::KeyS) {
            if let Some(ref world_state) = game_state.world_state {
                if let Some(player) = world_state.get_player(my_id) {
                    target_position = Some(Position::new(player.position.x, player.position.y + 1));
                    moved = true;
                }
            }
        }

        if keyboard_input.just_pressed(KeyCode::ArrowLeft) || keyboard_input.just_pressed(KeyCode::KeyA) {
            if let Some(ref world_state) = game_state.world_state {
                if let Some(player) = world_state.get_player(my_id) {
                    target_position = Some(Position::new(player.position.x - 1, player.position.y));
                    moved = true;
                }
            }
        }

        if keyboard_input.just_pressed(KeyCode::ArrowRight) || keyboard_input.just_pressed(KeyCode::KeyD) {
            if let Some(ref world_state) = game_state.world_state {
                if let Some(player) = world_state.get_player(my_id) {
                    target_position = Some(Position::new(player.position.x + 1, player.position.y));
                    moved = true;
                }
            }
        }

        if moved {
            if let Some(pos) = target_position {
                network_events.send(crate::network::NetworkEvent::SendMove(my_id, pos));
            }
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            network_events.send(crate::network::NetworkEvent::EndTurn(my_id));
        }
    }
}


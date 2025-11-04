use shared::protocol::{PlayerId, PlayerState, Position, WorldState};

/// Gestion de la logique du jeu côté serveur
pub struct Game {
    world_state: WorldState,
    player_counter: PlayerId,
}

impl Game {
    pub fn new(map_width: i32, map_height: i32) -> Self {
        Self {
            world_state: WorldState::new(map_width, map_height),
            player_counter: 1,
        }
    }

    /// Ajoute un nouveau joueur au jeu
    pub fn add_player(&mut self, position: Position) -> PlayerId {
        let player_id = self.player_counter;
        self.player_counter += 1;

        let player = PlayerState::new(player_id, position);
        self.world_state.players.push(player);

        player_id
    }

    /// Retire un joueur du jeu
    pub fn remove_player(&mut self, player_id: PlayerId) -> bool {
        if let Some(pos) = self.world_state.players.iter().position(|p| p.id == player_id) {
            self.world_state.players.remove(pos);
            true
        } else {
            false
        }
    }

    /// Déplace un joueur vers une nouvelle position
    pub fn move_player(&mut self, player_id: PlayerId, target: Position) -> Result<(), String> {
        // Vérifie d'abord les contraintes avant de modifier
        let (current_pos, movement_points, map_width, map_height) = {
            let player = self
                .world_state
                .get_player(player_id)
                .ok_or_else(|| "Joueur introuvable".to_string())?;

            if !player.is_alive {
                return Err("Le joueur est mort".to_string());
            }

            let distance = player.position.manhattan_distance(&target);
            if distance > player.movement_points as i32 {
                return Err("Pas assez de PM".to_string());
            }

            (
                player.position,
                player.movement_points,
                self.world_state.map_width,
                self.world_state.map_height,
            )
        };

        // Vérifie la position
        if target.x < 0
            || target.x >= map_width
            || target.y < 0
            || target.y >= map_height
        {
            return Err("Position invalide".to_string());
        }

        // Vérifie si la position est occupée
        let is_occupied = self
            .world_state
            .players
            .iter()
            .any(|p| p.position == target && p.is_alive && p.id != player_id);

        if is_occupied {
            return Err("Position occupée".to_string());
        }

        // Maintenant modifie le joueur
        let player = self
            .world_state
            .get_player_mut(player_id)
            .ok_or_else(|| "Joueur introuvable".to_string())?;

        let distance = current_pos.manhattan_distance(&target);
        player.position = target;
        player.movement_points = movement_points.saturating_sub(distance as u32);

        Ok(())
    }

    /// Gère une attaque entre deux joueurs
    pub fn attack(&mut self, attacker_id: PlayerId, target_id: PlayerId) -> Result<u32, String> {
        if attacker_id == target_id {
            return Err("Ne peut pas s'attaquer soi-même".to_string());
        }

        // Clone les données nécessaires pour éviter les conflits de borrow
        let attacker_position = self
            .world_state
            .get_player(attacker_id)
            .ok_or_else(|| "Attaquant introuvable".to_string())
            .map(|p| (p.position, p.is_alive, p.action_points))?;

        if !attacker_position.1 {
            return Err("L'attaquant est mort".to_string());
        }

        if attacker_position.2 == 0 {
            return Err("Pas assez de PA".to_string());
        }

        let target = self
            .world_state
            .get_player_mut(target_id)
            .ok_or_else(|| "Cible introuvable".to_string())?;

        if !target.is_alive {
            return Err("La cible est déjà morte".to_string());
        }

        let distance = attacker_position.0.manhattan_distance(&target.position);
        if distance > 1 {
            return Err("Cible hors de portée".to_string());
        }

        // Calcul des dégâts (simple pour l'instant)
        let damage = 25;
        target.health = target.health.saturating_sub(damage);

        if target.health == 0 {
            target.is_alive = false;
        }

        // Consomme les PA de l'attaquant
        if let Some(attacker_mut) = self.world_state.get_player_mut(attacker_id) {
            attacker_mut.action_points -= 1;
        }

        Ok(damage)
    }

    /// Termine le tour d'un joueur et passe au suivant
    pub fn end_turn(&mut self, player_id: PlayerId) -> Result<(), String> {
        if self.world_state.current_turn != player_id {
            return Err("Ce n'est pas votre tour".to_string());
        }

        if self.world_state.get_player(player_id).is_none() {
            return Err("Joueur introuvable".to_string());
        }

        // Passe au joueur suivant
        let alive_players: Vec<PlayerId> = self
            .world_state
            .players
            .iter()
            .filter(|p| p.is_alive)
            .map(|p| p.id)
            .collect();

        if let Some(current_pos) = alive_players.iter().position(|&id| id == player_id) {
            let next_pos = (current_pos + 1) % alive_players.len();
            self.world_state.current_turn = alive_players[next_pos];
            self.world_state.turn_number += 1;

            // Réinitialise les PA/PM du nouveau joueur
            if let Some(next_player) = self.world_state.get_player_mut(self.world_state.current_turn) {
                next_player.reset_turn();
            }
        }

        Ok(())
    }

    /// Vérifie si une position est valide (dans les limites de la carte)
    fn is_valid_position(&self, pos: &Position) -> bool {
        pos.x >= 0
            && pos.x < self.world_state.map_width
            && pos.y >= 0
            && pos.y < self.world_state.map_height
    }

    /// Vérifie si une position est occupée par un autre joueur
    fn is_position_occupied(&self, pos: &Position, exclude_id: Option<PlayerId>) -> bool {
        self.world_state
            .players
            .iter()
            .any(|p| p.position == *pos && p.is_alive && Some(p.id) != exclude_id)
    }

    /// Obtient l'état du monde
    pub fn get_world_state(&self) -> &WorldState {
        &self.world_state
    }

    /// Obtient une copie de l'état du monde
    pub fn get_world_state_clone(&self) -> WorldState {
        self.world_state.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = Game::new(10, 10);
        assert_eq!(game.world_state.map_width, 10);
        assert_eq!(game.world_state.map_height, 10);
    }

    #[test]
    fn test_add_player() {
        let mut game = Game::new(10, 10);
        let player_id = game.add_player(Position::new(5, 5));
        assert_eq!(player_id, 1);
        assert_eq!(game.world_state.players.len(), 1);
    }

    #[test]
    fn test_move_player() {
        let mut game = Game::new(10, 10);
        let player_id = game.add_player(Position::new(5, 5));
        assert!(game.move_player(player_id, Position::new(6, 5)).is_ok());
        assert_eq!(game.world_state.get_player(player_id).unwrap().position, Position::new(6, 5));
    }
}


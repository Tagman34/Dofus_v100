# Dofus-like - Jeu Multijoueur Tour par Tour

Un jeu multijoueur tour par tour inspiré de Dofus, développé en Rust avec Bevy pour le client et Tokio pour le serveur.

## 📋 Description

Ce projet est un jeu tactique multijoueur en temps réel où les joueurs peuvent :
- Se déplacer sur une carte en grille (10x10)
- Attaquer d'autres joueurs
- Gérer leurs points d'action (PA) et points de mouvement (PM)
- Jouer en tour par tour avec synchronisation réseau

## 🏗️ Architecture

Le projet est divisé en trois modules Rust :

### `shared/` 
Module partagé contenant le protocole réseau et les structures de données communes.
- **Position** : Position sur la grille (x, y)
- **PlayerState** : État d'un joueur (position, PA, PM, santé)
- **WorldState** : État global du monde de jeu
- **Message** : Messages réseau (Move, Attack, EndTurn, Sync, etc.)

### `server/`
Serveur de jeu asynchrone utilisant Tokio.
- Gestion des connexions TCP multiples
- Logique de jeu (déplacements, attaques, tours)
- Synchronisation de l'état du monde
- Persistance PostgreSQL optionnelle

### `client/`
Client graphique utilisant Bevy 0.13.
- Rendu 3D isométrique de la carte
- Interface utilisateur avec bevy_egui
- Connexion TCP au serveur
- Gestion des entrées clavier (WASD/Flèches)

## 🚀 Installation

### Prérequis

- **Rust** 1.87.0 ou supérieur
- **PostgreSQL** 14+ (optionnel, pour la persistance)
- **Cargo** (inclus avec Rust)

### Installation de Rust

```bash
# Installez Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Ou sur Windows, téléchargez depuis https://rustup.rs/
```

### Installation de PostgreSQL (optionnel)

```bash
# Ubuntu/Debian
sudo apt install postgresql postgresql-contrib

# macOS
brew install postgresql

# Windows
# Téléchargez depuis https://www.postgresql.org/download/windows/
```

## 🔧 Configuration

### Configuration de la base de données (optionnel)

Si vous souhaitez utiliser la persistance PostgreSQL :

1. Créez une base de données :
```sql
CREATE DATABASE dofus_game;
```

2. Créez un fichier `.env` à la racine du projet :
```env
DATABASE_URL=postgresql://username:password@localhost:5432/dofus_game
PORT=8080
RUST_LOG=info
```

3. Les migrations seront exécutées automatiquement au démarrage du serveur.

### Sans base de données

Le serveur peut fonctionner sans PostgreSQL. Dans ce cas, les données ne seront pas persistées entre les redémarrages.

## 🎮 Utilisation

### Lancer le serveur

```bash
# Terminal 1 - Lancer le serveur
cargo run -p server

# Le serveur démarre sur 0.0.0.0:8080 par défaut
```

### Lancer le client

```bash
# Terminal 2 - Lancer le premier client
cargo run -p client

# Terminal 3 - Lancer un second client (optionnel)
cargo run -p client
```

## 🎯 Contrôles

- **Flèches** ou **WASD** : Déplacer le personnage
- **Espace** : Terminer son tour

## 🧪 Tests

Le projet inclut des tests unitaires complets :

```bash
# Tester tout le workspace
cargo test --workspace

# Tester un module spécifique
cargo test -p shared
cargo test -p server --bin server

# Lancer les tests avec logs
RUST_LOG=debug cargo test --workspace
```

## 📊 Structure du projet

```
Dofus_V100/
├── Cargo.toml              # Workspace configuration
├── README.md               # Ce fichier
├── README_suivi.md         # Suivi détaillé du développement
│
├── shared/                 # Module partagé (protocole réseau)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs          # Position, PlayerState, WorldState, Messages
│
├── server/                 # Serveur de jeu
│   ├── Cargo.toml
│   ├── migrations/         # Migrations SQL PostgreSQL
│   │   └── 001_init.sql
│   └── src/
│       ├── main.rs         # Point d'entrée du serveur
│       ├── game.rs         # Logique du jeu
│       ├── handler.rs      # Gestion des messages
│       ├── session.rs      # Gestion des sessions
│       └── database/       # Module base de données
│           ├── mod.rs
│           ├── models.rs
│           └── queries.rs
│
└── client/                 # Client graphique Bevy
    ├── Cargo.toml
    └── src/
        ├── main.rs         # Point d'entrée + configuration Bevy
        ├── game.rs         # Logique client et rendu
        ├── network.rs      # Communication TCP
        └── ui.rs           # Interface utilisateur (egui)
```

## 🛠️ Technologies utilisées

### Serveur
- **Tokio** : Runtime asynchrone
- **SQLx** : Client PostgreSQL asynchrone
- **Serde** : Sérialisation/Désérialisation
- **Bincode** : Format binaire pour les messages réseau
- **Env_logger** : Logging

### Client
- **Bevy 0.13** : Moteur de jeu
- **bevy_egui 0.25** : Interface utilisateur
- **Tokio** : Communication réseau asynchrone

### Shared
- **Serde** : Sérialisation
- **Bincode** : Encodage binaire

## 🔐 Base de données

### Schéma

Le schéma PostgreSQL inclut les tables suivantes :

- **users** : Comptes utilisateurs
- **characters** : Personnages des joueurs
- **maps** : Cartes de jeu
- **fights** : Combats en cours
- **fight_participants** : Participants aux combats
- **inventory** : Inventaire des personnages
- **character_stats** : Statistiques des personnages

### Migrations

Les migrations SQL sont dans `server/migrations/` et sont exécutées automatiquement au démarrage du serveur.

## 📈 État du développement

✅ **Fonctionnalités implémentées :**
- Serveur TCP multithread avec Tokio
- Client graphique 3D avec Bevy
- Système de déplacement tour par tour
- Système de combat (attaques au corps à corps)
- Gestion des PA/PM
- Synchronisation réseau
- Persistance PostgreSQL (infrastructure complète)
- Tests unitaires complets

🚧 **En cours / À venir :**
- Authentification des joueurs
- Système de compétences/sorts
- Cartes variées et donjons
- Inventaire et équipement
- Système de progression (levels, XP)
- IA pour NPCs

## 📝 Développement

### Compiler le projet

```bash
# Vérifier la compilation
cargo check --workspace

# Compiler en mode debug
cargo build --workspace

# Compiler en mode release (optimisé)
cargo build --workspace --release
```

### Formater le code

```bash
cargo fmt --all
```

### Linter (clippy)

```bash
cargo clippy --all-targets --all-features
```

### Documentation

```bash
# Générer et ouvrir la documentation
cargo doc --open --no-deps
```

## 🤝 Contribution

Ce projet est un projet personnel d'apprentissage. Les contributions ne sont pas acceptées pour le moment.

## 📄 Licence

Ce projet est développé à des fins éducatives.

## 🐛 Problèmes connus

- Le client utilise des APIs dépréciées de Bevy (shapes) - compatible avec Bevy 0.13
- La synchronisation réseau est simplifiée (pas de prédiction côté client)
- Pas de reconnexion automatique en cas de déconnexion

## 🔗 Ressources

- [Documentation Rust](https://doc.rust-lang.org/)
- [Documentation Bevy](https://bevyengine.org/learn/)
- [Documentation Tokio](https://tokio.rs/)
- [Documentation SQLx](https://docs.rs/sqlx/)

---

**Dernière mise à jour** : Novembre 2025


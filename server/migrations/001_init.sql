-- Schéma de base de données pour le jeu Dofus-like

-- Table des utilisateurs
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT TRUE
);

-- Table des personnages
CREATE TABLE IF NOT EXISTS characters (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(50) UNIQUE NOT NULL,
    level INTEGER DEFAULT 1 CHECK (level >= 1 AND level <= 200),
    experience BIGINT DEFAULT 0 CHECK (experience >= 0),
    health INTEGER DEFAULT 100 CHECK (health >= 0),
    max_health INTEGER DEFAULT 100 CHECK (max_health > 0),
    action_points INTEGER DEFAULT 6 CHECK (action_points >= 0),
    movement_points INTEGER DEFAULT 3 CHECK (movement_points >= 0),
    position_x INTEGER DEFAULT 0,
    position_y INTEGER DEFAULT 0,
    map_id INTEGER,
    is_alive BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_played TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table des cartes/maps
CREATE TABLE IF NOT EXISTS maps (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    width INTEGER NOT NULL CHECK (width > 0),
    height INTEGER NOT NULL CHECK (height > 0),
    map_type VARCHAR(50) DEFAULT 'normal', -- normal, dungeon, pvp, etc.
    difficulty_level INTEGER DEFAULT 1 CHECK (difficulty_level >= 1 AND difficulty_level <= 10),
    is_pvp_enabled BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table des combats
CREATE TABLE IF NOT EXISTS fights (
    id SERIAL PRIMARY KEY,
    map_id INTEGER REFERENCES maps(id) ON DELETE SET NULL,
    fight_type VARCHAR(50) DEFAULT 'pvm', -- pvm, pvp, boss
    current_turn INTEGER DEFAULT 1,
    turn_number INTEGER DEFAULT 1,
    is_active BOOLEAN DEFAULT TRUE,
    started_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMP WITH TIME ZONE,
    winner_id INTEGER REFERENCES characters(id) ON DELETE SET NULL
);

-- Table de liaison entre combats et participants
CREATE TABLE IF NOT EXISTS fight_participants (
    id SERIAL PRIMARY KEY,
    fight_id INTEGER NOT NULL REFERENCES fights(id) ON DELETE CASCADE,
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    team INTEGER DEFAULT 1 CHECK (team IN (1, 2)), -- Team 1 ou Team 2
    is_alive BOOLEAN DEFAULT TRUE,
    damage_dealt INTEGER DEFAULT 0,
    damage_taken INTEGER DEFAULT 0,
    turns_played INTEGER DEFAULT 0,
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    left_at TIMESTAMP WITH TIME ZONE,
    UNIQUE(fight_id, character_id)
);

-- Table des inventaires
CREATE TABLE IF NOT EXISTS inventory (
    id SERIAL PRIMARY KEY,
    character_id INTEGER NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    item_type VARCHAR(50) NOT NULL,
    item_name VARCHAR(100) NOT NULL,
    quantity INTEGER DEFAULT 1 CHECK (quantity > 0),
    properties JSONB, -- Propriétés spécifiques des objets (stats, etc.)
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Table des statistiques de personnages
CREATE TABLE IF NOT EXISTS character_stats (
    id SERIAL PRIMARY KEY,
    character_id INTEGER UNIQUE NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    strength INTEGER DEFAULT 0,
    intelligence INTEGER DEFAULT 0,
    agility INTEGER DEFAULT 0,
    vitality INTEGER DEFAULT 0,
    wisdom INTEGER DEFAULT 0,
    chance INTEGER DEFAULT 0,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Index pour améliorer les performances
CREATE INDEX IF NOT EXISTS idx_characters_user_id ON characters(user_id);
CREATE INDEX IF NOT EXISTS idx_characters_map_id ON characters(map_id);
CREATE INDEX IF NOT EXISTS idx_fights_map_id ON fights(map_id);
CREATE INDEX IF NOT EXISTS idx_fights_is_active ON fights(is_active);
CREATE INDEX IF NOT EXISTS idx_fight_participants_fight_id ON fight_participants(fight_id);
CREATE INDEX IF NOT EXISTS idx_fight_participants_character_id ON fight_participants(character_id);
CREATE INDEX IF NOT EXISTS idx_inventory_character_id ON inventory(character_id);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- Insertion de données de test
INSERT INTO maps (name, width, height, map_type, difficulty_level) VALUES
    ('Plaine des débutants', 10, 10, 'normal', 1),
    ('Forêt sombre', 15, 15, 'normal', 3),
    ('Arène PvP', 12, 12, 'pvp', 5),
    ('Donjon du Dragon', 20, 20, 'dungeon', 10)
ON CONFLICT DO NOTHING;


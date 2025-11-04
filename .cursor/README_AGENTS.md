# Guide d'utilisation du système multi-agents

## 📋 Fichiers de configuration

### 1. `.cursor/config.json` - Configuration technique
Ce fichier définit la structure technique du projet et les agents disponibles :

**Structure du projet :**
- ✅ Workspace Rust avec 3 membres : `server`, `client`, `shared`
- ✅ Edition Rust 2021
- ✅ Formatage : rustfmt + clippy

**Agents configurés :**
1. **SharedAgent** (Priorité 1) - Développe le module `shared/lib.rs`
2. **BackendAgent** (Priorité 2) - Développe `server/src/**`
3. **DatabaseAgent** (Priorité 3) - Gère `server/src/database/**` et migrations
4. **FrontendAgent** (Priorité 2) - Développe `client/src/**`
5. **TestAgent** (Priorité 1) - Valide avec tests, fmt, clippy

**Configuration d'exécution :**
- Agents en parallèle activés
- 3 agents maximum en simultané
- Auto-commit activé
- Synchronisation après chaque étape

### 2. `.cursor/task.yaml` - Plan de travail détaillé
Ce fichier définit le workflow complet du projet avec les étapes et responsabilités de chaque agent.

**Workflow en 6 étapes :**
1. Initialisation du projet (✅ FAIT)
2. Protocole réseau (SharedAgent)
3. Serveur (BackendAgent)
4. Client (FrontendAgent)
5. Combat tour par tour (BackendAgent + FrontendAgent)
6. Base de données (DatabaseAgent)
7. Tests & Documentation (TestAgent)

## 🚀 Comment utiliser le système multi-agents

### Option 1 : Utilisation via Cursor (Recommandé)

Les fichiers `.cursor/config.json` et `.cursor/task.yaml` sont automatiquement détectés par Cursor. Vous pouvez :

1. **Activer le mode multi-agents dans Cursor** :
   - Ouvrir le panneau des agents
   - Sélectionner les agents à activer
   - Assigner des tâches depuis le workflow défini dans `task.yaml`

2. **Exécuter une étape du workflow** :
   ```
   "Implémenter l'étape 2 : Protocole réseau"
   ```
   → Cursor utilisera automatiquement le **SharedAgent** selon la configuration

3. **Exécuter plusieurs agents en parallèle** :
   ```
   "Développer le serveur et le client en parallèle"
   ```
   → Cursor lancera **BackendAgent** et **FrontendAgent** simultanément

### Option 2 : Utilisation manuelle via prompts

Vous pouvez invoquer directement les agents en mentionnant leur rôle :

**Exemples de prompts :**

```
"En tant que SharedAgent, développe le protocole réseau complet avec tous les messages nécessaires"
```

```
"BackendAgent, implémente le serveur Tokio avec gestion des connexions multiples"
```

```
"DatabaseAgent, crée le schéma PostgreSQL et les migrations SQLx"
```

```
"FrontendAgent, crée la scène Bevy avec caméra isométrique et affichage de la carte"
```

```
"TestAgent, vérifie que tout compile et passe tous les tests"
```

### Option 3 : Utilisation séquentielle (étape par étape)

Suivre le workflow défini dans `task.yaml` :

1. **Étape 1 - Initialisation** : ✅ COMPLÉTÉE
2. **Étape 2 - Protocole réseau** :
   ```
   "SharedAgent, implémente les structures Message, PlayerState, WorldState dans shared/lib.rs"
   ```
3. **Étape 3 - Serveur** :
   ```
   "BackendAgent, développe le serveur Tokio avec la logique tour par tour"
   ```
4. **Étape 4 - Client** :
   ```
   "FrontendAgent, crée l'interface Bevy avec connexion au serveur"
   ```
5. **Étape 5 - Combat** :
   ```
   "BackendAgent et FrontendAgent, implémentez le système de combat tour par tour"
   ```
6. **Étape 6 - Base de données** :
   ```
   "DatabaseAgent, crée le schéma SQL et les fonctions de persistance"
   ```
7. **Étape 7 - Tests** :
   ```
   "TestAgent, valide tout le projet avec tests, fmt et clippy"
   ```

## 📊 Vérification de l'état actuel

### ✅ Structure actuelle (conforme à la config)

```
Dofus_V100/
├── Cargo.toml              ✅ Workspace configuré
├── server/
│   ├── Cargo.toml         ✅ Dépendances : tokio, serde, sqlx, etc.
│   └── src/main.rs        ✅ Point d'entrée basique
├── client/
│   ├── Cargo.toml         ✅ Dépendances : bevy, bevy_egui, etc.
│   └── src/main.rs        ✅ Point d'entrée basique
└── shared/
    ├── Cargo.toml         ✅ Dépendances : serde, bincode
    └── src/lib.rs         ✅ Module protocol basique
```

### ⚠️ À développer par les agents

1. **SharedAgent** doit développer :
   - Structures complètes : `PlayerState`, `WorldState`, `GameState`
   - Messages réseau : `Move`, `Attack`, `EndTurn`, `Sync`, etc.
   - Sérialisation/désérialisation complète

2. **BackendAgent** doit développer :
   - Serveur TCP/WebSocket avec Tokio
   - Gestion des sessions et connexions multiples
   - Logique du jeu tour par tour
   - Intégration avec la base de données

3. **DatabaseAgent** doit développer :
   - Schéma SQL (users, characters, maps, fights)
   - Migrations SQLx
   - Fonctions de persistance dans `server/src/database/`

4. **FrontendAgent** doit développer :
   - Scène Bevy avec caméra isométrique
   - Affichage de la carte 10x10
   - Gestion des entrées utilisateur
   - Connexion au serveur
   - Interface bevy_egui

5. **TestAgent** doit :
   - Écrire des tests unitaires
   - Vérifier le formatage et les warnings
   - Valider la compilation

## 🔧 Commandes utiles

### Vérifier la compilation
```bash
cargo check --workspace
```

### Formater le code
```bash
cargo fmt --all
```

### Vérifier les warnings
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Lancer les tests
```bash
cargo test --workspace
```

### Lancer le serveur
```bash
cargo run -p server
```

### Lancer le client
```bash
cargo run -p client
```

## 📝 Notes importantes

1. **Ordre de développement** : Les agents respectent les priorités définies dans `config.json`
   - Priorité 1 : SharedAgent, TestAgent (doivent être développés en premier)
   - Priorité 2 : BackendAgent, FrontendAgent (peuvent être développés en parallèle)
   - Priorité 3 : DatabaseAgent (dépend de BackendAgent)

2. **Synchronisation** : Le système auto-commit et sync après chaque étape pour éviter les conflits

3. **Logs** : Les logs des agents sont disponibles dans `.cursor/logs/`

4. **Statut** : Le fichier `.cursor/status.txt` indique l'état actuel du projet

## 🎯 Prochaines étapes recommandées

1. **Démarrer avec SharedAgent** :
   ```
   "SharedAgent, développe le protocole réseau complet selon les spécifications dans task.yaml"
   ```

2. **Puis BackendAgent et FrontendAgent en parallèle** :
   ```
   "BackendAgent, développe le serveur Tokio"
   "FrontendAgent, développe le client Bevy"
   ```

3. **Ensuite DatabaseAgent** :
   ```
   "DatabaseAgent, configure PostgreSQL et crée les migrations"
   ```

4. **Enfin TestAgent pour valider** :
   ```
   "TestAgent, valide tout le projet"
   ```

---

**Le système multi-agents est prêt à être utilisé !** 🚀


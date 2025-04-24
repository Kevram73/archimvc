# ArchiMVC

Une application REST API construite avec Rust, Actix-web et Diesel, utilisant SQLite comme base de données. Le projet suit une architecture MVC (Model-View-Controller) pour une meilleure organisation et scalabilité.

## Architecture du Projet

Le projet est organisé selon le pattern MVC :

```
src/
├── models/          # Définition des structures de données et logique métier
│   ├── user.rs
│   └── mod.rs
├── controllers/     # Gestion des requêtes et réponses HTTP
│   ├── user_controller.rs
│   └── mod.rs
├── routes/         # Configuration des routes de l'API
│   └── mod.rs
└── main.rs
```

Cette architecture permet de :
- Séparer les responsabilités
- Faciliter l'ajout de nouveaux endpoints
- Améliorer la maintenabilité
- Structurer le code de manière cohérente

## Prérequis

- Docker
- Docker Compose

## Installation

1. Cloner le dépôt :
```bash
git clone https://github.com/Kevram73/archimvc.git
cd archimvc
```

2. Construire l'image Docker :
```bash
docker build -t archimvc .
```

3. Lancer l'application avec Docker Compose :
```bash
# Démarrer les conteneurs en arrière-plan
docker-compose up -d

# Ou pour voir les logs en temps réel
docker-compose up
```

4. Vérifier que l'application est en cours d'exécution :
```bash
docker-compose ps
```

5. Pour arrêter l'application :
```bash
docker-compose down
```

L'application sera accessible sur `http://localhost:8080`

## Configuration

La base de données SQLite sera automatiquement créée au premier lancement dans le volume Docker `sqlite_data`.

## Endpoints API

### Users
- `GET /users` - Récupérer tous les utilisateurs
- `GET /users/{id}` - Récupérer un utilisateur par ID
- `POST /users` - Créer un nouvel utilisateur
- `PUT /users/{id}` - Mettre à jour un utilisateur
- `DELETE /users/{id}` - Supprimer un utilisateur

Pour ajouter de nouveaux endpoints :
1. Créer le modèle dans `models/`
2. Implémenter le contrôleur dans `controllers/`
3. Configurer les routes dans `routes/`

## Développement

Pour développer localement sans Docker :

1. Installer Rust et SQLite
2. Configurer la variable d'environnement :
```bash
export DATABASE_URL=sqlite://archimvc.db
```
3. Exécuter :
```bash
cargo run
```

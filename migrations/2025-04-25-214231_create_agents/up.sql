-- Your SQL goes here
CREATE TABLE agents (
            id INTEGER PRIMARY KEY NOT NULL,
            nom TEXT,
            phone TEXT,
            code TEXT NOT NULL,
            parent_id INTEGER REFERENCES agents(id),
            profil_id INTEGER REFERENCES profiles(id),
            taux_commission INTEGER NOT NULL DEFAULT 0,
            password TEXT NOT NULL,
            actif BOOLEAN NOT NULL DEFAULT 1,
            membre BOOLEAN NOT NULL DEFAULT 0,
            pwd_changed BOOLEAN NOT NULL DEFAULT 0,
            fils_parent BOOLEAN NOT NULL DEFAULT 0,
            code_a TEXT NOT NULL,
            code_b TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
)
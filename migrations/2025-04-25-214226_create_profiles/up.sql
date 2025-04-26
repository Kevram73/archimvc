-- Your SQL goes here
CREATE TABLE profiles (
            id INTEGER PRIMARY KEY NOT NULL,
            nom TEXT,
            fils_parent BOOLEAN NOT NULL DEFAULT 0,
            taux_commission INTEGER NOT NULL DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
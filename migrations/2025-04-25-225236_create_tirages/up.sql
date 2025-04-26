CREATE TABLE IF NOT EXISTS tirages (
    id INTEGER PRIMARY KEY NOT NULL,
    jeu_id INTEGER,
    numero INTEGER,
    total INTEGER NOT NULL,
    la_date DATE,
    hr_lance TIMESTAMP,
    hr_seuil TIME,
    hr_clot TIME,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS tirages_jeu_id_idx ON tirages(jeu_id);
CREATE INDEX IF NOT EXISTS tirages_la_date_idx ON tirages(la_date);

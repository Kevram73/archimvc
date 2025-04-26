-- Your SQL goes here
CREATE TABLE gains (
    id INTEGER PRIMARY KEY NOT NULL,
    nb_boules INTEGER,
    nb_pos_fixe INTEGER,
    facteur INTEGER,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
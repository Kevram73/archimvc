-- This file should undo anything in `up.sql`

DROP INDEX IF EXISTS tirages_jeu_id_idx;
DROP INDEX IF EXISTS tirages_la_date_idx;
DROP TABLE IF EXISTS tirages;

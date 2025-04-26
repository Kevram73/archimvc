use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::Selectable;
use serde::{Deserialize, Serialize};

use crate::schema::jeux;


#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::jeux)]
pub struct Jeu {
    pub id: i32,
    pub nom:     String,
    pub pays_id: i32,
    pub actif:   bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = jeux)]
pub struct NewJeu {
    pub nom:     String,
    pub pays_id: i32,
    pub actif:   bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// If you want to validate data on creation:
impl NewJeu {
    pub fn validate(&self) -> Result<(), String> {
        if self.nom.trim().is_empty() {
            return Err("Name cannot be empty".into());
        }
        Ok(())
    }
}

#[derive(Debug, AsChangeset, Deserialize)]
#[diesel(table_name = jeux)]
pub struct UpdateJeu {
    pub nom:     String,
    pub pays_id: i32,
    pub actif:   bool,
    #[serde(skip_deserializing)]
    pub updated_at:  Option<NaiveDateTime>,
}

impl UpdateJeu {
    pub fn prepare(mut self) -> Self {
        self.updated_at = Some(Utc::now().naive_utc());
        self
    }
}
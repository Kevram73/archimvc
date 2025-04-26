use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::agents)]
pub struct Agent {
    pub id: i32,
    pub nom: Option<String>,
    pub phone: Option<String>,
    pub code: String,
    pub parent_id: Option<i32>,
    pub profil_id: Option<i32>,
    pub taux_commission: i32,
    pub password: String,
    pub actif: bool,
    pub membre: bool,
    pub pwd_changed: bool,
    pub fils_parent: bool,
    pub code_a: String,
    pub code_b: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::agents)]
pub struct NewAgent {
    pub nom: Option<String>,
    pub phone: Option<String>,
    pub code: String,
    pub parent_id: Option<i32>,
    pub profil_id: Option<i32>,
    pub taux_commission: i32,
    pub password: String,
    pub actif: bool,
    pub membre: bool,
    pub pwd_changed: bool,
    pub fils_parent: bool,
    pub code_a: String,
    pub code_b: String,
}


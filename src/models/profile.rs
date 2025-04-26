use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::profiles)]
pub struct Profile {
    pub id: i32,
    pub nom: Option<String>,
    pub fils_parent: bool,
    pub taux_commission: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::profiles)]
pub struct NewProfile {
    pub nom: Option<String>,
    pub fils_parent: bool,
    pub taux_commission: i32,
}

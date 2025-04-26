use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::gains;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Gain {
    pub id: i32,
    pub nb_boules: Option<i32>,
    pub nb_pos_fixe: Option<i32>,
    pub facteur: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = gains)]
pub struct NewGain {
    pub nb_boules: Option<i32>,
    pub nb_pos_fixe: Option<i32>,
    pub facteur: Option<i32>,
}

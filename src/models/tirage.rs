use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tirages)]
pub struct Tirage {
    pub id: i32,
    pub jeu_id: Option<i32>,
    pub numero: Option<i32>,
    pub total: i32,
    pub la_date: Option<NaiveDate>,
    pub hr_lance: Option<NaiveDateTime>,
    pub hr_seuil: Option<NaiveTime>,
    pub hr_clot: Option<NaiveTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::tirages)]
pub struct NewTirage {
    pub jeu_id: Option<i32>,
    pub numero: Option<i32>,
    pub total: i32,
    pub la_date: Option<NaiveDate>,
    pub hr_lance: Option<NaiveDateTime>,
    pub hr_seuil: Option<NaiveTime>,
    pub hr_clot: Option<NaiveTime>,
}
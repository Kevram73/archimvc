use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::users;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Debug, Clone, Serialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[diesel(sql_type = diesel::sql_types::Timestamp)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub username: String,
    pub email: String,
}

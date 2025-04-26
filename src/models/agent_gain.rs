use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::agent_gains)]
pub struct AgentGain {
    pub id: i32,
    pub gain_id: i32,
    pub agent_id: i32,
    pub facteur: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::agent_gains)]
pub struct NewAgentGain {
    pub gain_id: i32,
    pub agent_id: i32,
    pub facteur: i32,
}
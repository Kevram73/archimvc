use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};

#[derive(Insertable)]
#[diesel(table_name = crate::schema::agent_gains)]
struct NewAgentGain {
    gain_id: i32,
    agent_id: i32,
    facteur: i32,
}

pub fn seed_agent_gains(pool: &Pool<ConnectionManager<SqliteConnection>>) {
    use crate::schema::agent_gains;
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let default_gains: Vec<NewAgentGain> = vec![
        NewAgentGain { agent_id: 1, gain_id: 1, facteur: 100 },
        NewAgentGain { agent_id: 1, gain_id: 2, facteur: 75 },
        NewAgentGain { agent_id: 2, gain_id: 1, facteur: 80 },
        NewAgentGain { agent_id: 2, gain_id: 2, facteur: 60 },
    ];

    diesel::insert_into(agent_gains::table)
        .values(&default_gains)
        .execute(conn)
        .expect("Error seeding agent gains");
    
    println!("✅ {} agent gains ont été créés avec succès!", default_gains.len());
}

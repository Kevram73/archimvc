use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::agent::NewAgent;

pub fn seed_agents(pool: &Pool<ConnectionManager<SqliteConnection>>) {
    use crate::schema::agents;
    let conn = &mut pool.get().expect("Failed to get DB connection");

    let default_agents: Vec<NewAgent> = (1..=10)
        .map(|i| NewAgent {
            nom: Some(format!("Agent {}", i)),
            phone: Some(format!("123456{:04}", i)),
            code: format!("SA{:03}", i),
            parent_id: None,
            profil_id: Some(2), // Profile standard
            taux_commission: 5,
            password: format!("password{}", i),
            actif: true,
            membre: true,
            pwd_changed: false,
            fils_parent: false,
            code_a: format!("A{:03}", i),
            code_b: format!("B{:03}", i),
        })
        .collect();

    // Ajout de l'agent admin séparément
    let admin_agent = NewAgent {
        nom: Some("Super Agent".to_string()),
        phone: Some("1234567890".to_string()),
        code: "SA001".to_string(),
        parent_id: None,
        profil_id: Some(1), // Admin profile
        taux_commission: 10,
        password: "password123".to_string(),
        actif: true,
        membre: true,
        pwd_changed: false,
        fils_parent: false,
        code_a: "A001".to_string(),
        code_b: "B001".to_string(),
    };

    diesel::insert_into(agents::table)
        .values(&admin_agent)
        .execute(conn)
        .expect("Error seeding admin agent");

    diesel::insert_into(agents::table)
        .values(&default_agents)
        .execute(conn)
        .expect("Error seeding agents");
    
    println!("✅ {} agents ont été créés avec succès!", default_agents.len() + 1);
}

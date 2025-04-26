use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::profile::NewProfile;

pub fn seed_profiles(pool: &Pool<ConnectionManager<SqliteConnection>>) {
    use crate::schema::profiles;
    let conn = &mut pool.get().expect("Échec de connexion à la BD");

    let default_profiles = vec![
        NewProfile {
            nom: Some("Admin".to_string()),
            fils_parent: false,
            taux_commission: 10,
        },
        NewProfile {
            nom: Some("Standard".to_string()),
            fils_parent: false,
            taux_commission: 5,
        },
    ];

    diesel::insert_into(profiles::table)
        .values(&default_profiles)
        .execute(conn)
        .expect("Erreur lors de la création des profils");
    
    println!("✅ {} profils ont été créés avec succès!", default_profiles.len());
}

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use fake::{Fake, Faker};
use crate::models::jeu::NewJeu;

pub fn seed_jeux(pool: &Pool<ConnectionManager<SqliteConnection>>) -> Result<(), Box<dyn std::error::Error>> {
    use crate::schema::jeux;
    let conn = &mut pool.get().expect("Failed to get DB connection");

    for _ in 0..10 {
        let nom: String = Faker.fake();
        let pays_id: i32 = (1..5).fake();
        let actif: i32 = 1;
        
        let nouveau_jeu = NewJeu {
            nom,
            pays_id,
            actif,
        };

        diesel::insert_into(jeux::table)
            .values(&nouveau_jeu)
            .execute(conn)?;
    }

    Ok(())
}

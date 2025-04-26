use diesel::prelude::*;
use crate::models::NewUser;
use crate::db::DbPool;
use crate::schema::users;

pub fn seed_users(pool: &DbPool) {
    let mut conn = pool.get().expect("Impossible d'obtenir une connexion");
    
    let users: Vec<NewUser> = (1..=50)
        .map(|i| {
            let new_user = NewUser {
                username: format!("user_{}", i),
                email: format!("user{}@example.com", i),
            };
            new_user
        })
        .collect();

    diesel::insert_into(users::table)
        .values(&users)
        .execute(&mut conn)
        .expect("Erreur lors du seeding des utilisateurs");

    println!("✅ 50 utilisateurs ont été créés avec succès!");
}

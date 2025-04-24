mod controllers;
mod models;
mod routes;
mod db;
mod schema;
mod seeders;

use actix_web::{App, HttpServer, web};
use routes::configure_routes;
use dotenvy::dotenv;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use seeders::seed_users;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::establish_connection();
    
    // Exécuter les migrations
    let mut conn = pool.get().expect("Failed to get DB connection");
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
    
    println!("✅ Migrations executed successfully!");

    // Exécuter les seeders
    if std::env::var("RUN_SEEDERS").unwrap_or_else(|_| "false".to_string()) == "true" {
        seed_users(&pool);
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

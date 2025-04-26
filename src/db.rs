use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::env;
use std::path::Path;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Créer le dossier db s'il n'existe pas
    if let Some(db_path) = Path::new(&database_url).parent() {
        std::fs::create_dir_all(db_path).expect("Failed to create db directory");
    }
    
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

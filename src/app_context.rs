use std::env;

use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct AppContext {
    pub db: DatabaseConnection,
}

impl AppContext {
    pub async fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = Database::connect(database_url)
            .await
            .expect("Database connection failed");

        Self { db }
    }
}

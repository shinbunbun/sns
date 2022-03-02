pub use sea_schema::migration::*;

mod m20220301_000001_create_user_table;
mod m20220301_000002_create_follows_table;
mod m20220301_000003_create_messages_table;
mod m20220301_000004_create_likes_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220301_000001_create_user_table::Migration),
            Box::new(m20220301_000002_create_follows_table::Migration),
            Box::new(m20220301_000003_create_messages_table::Migration),
            Box::new(m20220301_000004_create_likes_table::Migration),
        ]
    }
}

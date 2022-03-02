use entity::user::*;
use sea_orm::{DbBackend, Schema};
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220301_000001_create_user_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Schema::new(DbBackend::MySql).create_table_from_entity(Entity),
                /* Table::create()
                .table(Entity)
                .if_not_exists()
                .col(
                    ColumnDef::new(Column::UserId)
                        .string_len(26)
                        .not_null()
                        .primary_key(),
                )
                .col(ColumnDef::new(Column::UserName).string_len(50).not_null())
                .col(ColumnDef::new(Column::EMail).string_len(254).not_null())
                .col(
                    ColumnDef::new(Column::PasswordHash)
                        .string_len(128)
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Column::CreatedAt)
                        .date_time()
                        .timestamp()
                        .not_null(),
                )
                .col(
                    ColumnDef::new(Column::UpdatedAt)
                        .date_time()
                        // .timestamp()
                        .not_null(),
                )
                .to_owned(), */
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}

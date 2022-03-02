use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[sea_orm(column_type = "String(Some(26))")]
    pub user_id: String,
    #[sea_orm(column_type = "String(Some(50))")]
    pub user_name: String,
    #[sea_orm(column_type = "String(Some(254))")]
    pub e_mail: String,
    #[sea_orm(column_type = "String(Some(128))")]
    pub password_hash: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

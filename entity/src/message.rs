use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "messages")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[sea_orm(column_type = "String(Some(26))")]
    pub message_id: String,
    #[sea_orm(column_type = "String(Some(26))")]
    pub user_id: String,
    #[sea_orm(column_type = "Text")]
    pub message_text: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::like::Entity")]
    Like,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::UserId"
    )]
    UserId,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserId.def()
    }
}

impl Related<super::like::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Like.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

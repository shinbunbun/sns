use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "follows")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    #[sea_orm(column_type = "String(Some(26))")]
    pub follow_user_id: String,
    #[sea_orm(primary_key, auto_increment = false)]
    #[sea_orm(column_type = "String(Some(26))")]
    pub follower_user_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::FollowUserId",
        to = "super::user::Column::UserId"
    )]
    FollowUserId,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::FollowerUserId",
        to = "super::user::Column::UserId"
    )]
    FollowerUserId,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FollowUserId.def()
    }
    fn via() -> Option<RelationDef> {
        Some(Relation::FollowerUserId.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

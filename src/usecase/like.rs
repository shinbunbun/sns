use sea_orm::{DbConn, DbErr, EntityTrait, InsertResult, Set};

pub async fn insert(
    db: &DbConn,
    user_id: &str,
    message_id: &str,
) -> Result<InsertResult<entity::like::ActiveModel>, DbErr> {
    let model = entity::like::ActiveModel {
        user_id: Set(user_id.to_owned()),
        message_id: Set(message_id.to_owned()),
    };
    entity::like::Entity::insert(model).exec(db).await
}

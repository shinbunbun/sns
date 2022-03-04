use sea_orm::{entity::*, error::*, DbConn, InsertResult};
use ulid::Ulid;

pub async fn insert(
    db: &DbConn,
    user_id: &str,
    message_text: &str,
) -> Result<InsertResult<entity::message::ActiveModel>, DbErr> {
    let ulid = Ulid::new().to_string();
    let model = entity::message::ActiveModel {
        message_id: Set(ulid.to_owned()),
        user_id: Set(user_id.to_owned()),
        message_text: Set(message_text.to_owned()),
        ..Default::default()
    };
    entity::message::Entity::insert(model).exec(db).await
}

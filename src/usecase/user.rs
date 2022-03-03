use crate::controller;
use sea_orm::{entity::*, error::*, DbConn, InsertResult};

pub async fn insert(
    db: &DbConn,
    user: &controller::signup::Req,
    ulid: &str,
    password_hash: &str,
) -> Result<InsertResult<entity::user::ActiveModel>, DbErr> {
    let model = entity::user::ActiveModel {
        user_id: Set(ulid.to_owned()),
        user_name: Set(user.name.to_owned()),
        e_mail: Set(user.email.to_owned()),
        password_hash: Set(password_hash.to_owned()),
        ..Default::default()
    };
    entity::user::Entity::insert(model).exec(db).await
}

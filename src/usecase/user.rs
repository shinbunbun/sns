use sea_orm::{entity::*, error::*, DbConn, InsertResult, QueryFilter};
use ulid::Ulid;

pub async fn insert(
    db: &DbConn,
    e_mail: &str,
    password_hash: &str,
    name: &str,
) -> Result<InsertResult<entity::user::ActiveModel>, DbErr> {
    let ulid = Ulid::new().to_string();
    let model = entity::user::ActiveModel {
        user_id: Set(ulid.to_owned()),
        user_name: Set(name.to_owned()),
        e_mail: Set(e_mail.to_owned()),
        password_hash: Set(password_hash.to_owned()),
        ..Default::default()
    };
    entity::user::Entity::insert(model).exec(db).await
}

pub async fn select_by_user_id(
    db: &DbConn,
    user_id: &str,
) -> Result<Option<entity::user::Model>, DbErr> {
    entity::user::Entity::find()
        .filter(entity::user::Column::UserId.eq(user_id))
        .one(db)
        .await
}

pub async fn select_by_e_mail(
    db: &DbConn,
    e_mail: &str,
) -> Result<Option<entity::user::Model>, DbErr> {
    entity::user::Entity::find()
        .filter(entity::user::Column::EMail.eq(e_mail))
        .one(db)
        .await
}

use sea_orm::{entity::*, error::*, DbConn, InsertResult};
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

use sea_orm::{
    entity::*, error::*, ConnectionTrait, DatabaseBackend, DbConn, InsertResult, QueryResult,
    Statement,
};
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

pub async fn select_all_posts_info(db: &DbConn, user_id: &str) -> Result<Vec<QueryResult>, DbErr> {
    let user_id = String::from("\"") + &String::from(user_id) + &String::from("\"");
    db.query_all(Statement::from_string(
        DatabaseBackend::MySql,
        [
            "SELECT `messages`.`message_id`, `users`.`user_name`, `messages`.`message_text`, `messages`.`created_at`, IFNULL(`X`.`likes_count`, 0) as likes_count,",
            "EXISTS(",
            "SELECT 1",
            "FROM `likes`",
            "WHERE `likes`.`message_id` = `messages`.`message_id`",
            "AND `likes`.`user_id` = ",
            &user_id,
            ") AS is_like",
            "FROM `messages`",
            "JOIN `users` ON `messages`.`user_id` = `users`.`user_id`",
            "LEFT JOIN(",
            "SELECT `message_id`, COUNT(*) AS `likes_count`",
            "FROM `likes`",
            "GROUP BY `message_id`",
            ")",
            "AS `X`",
            "ON `messages`.`message_id`=`X`.`message_id`",
        ].join(" ").to_owned(),
    ))
    .await
}

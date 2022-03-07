use sea_orm::{
    entity::*, error::*, prelude::DateTimeWithTimeZone, ConnectionTrait, DbConn, FromQueryResult,
    InsertResult, Statement,
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

pub async fn select_all_messages_info(db: &DbConn, user_id: &str) -> Result<Vec<Message>, DbErr> {
    let user_id = String::from("\"") + &String::from(user_id) + &String::from("\"");
    Message::find_by_statement(Statement::from_sql_and_values(
        db.get_database_backend(),
        &[
            "SELECT `messages`.`message_id`, `users`.`user_name`, `messages`.`message_text`, `messages`.`created_at`, IFNULL(`X`.`likes`, 0) as likes,",
            "EXISTS(",
            "SELECT 1",
            "FROM `likes`",
            "WHERE `likes`.`message_id` = `messages`.`message_id`",
            "AND `likes`.`user_id` = ?",
            ") AS is_like",
            "FROM `messages`",
            "JOIN `users` ON `messages`.`user_id` = `users`.`user_id`",
            "LEFT JOIN(",
            "SELECT `message_id`, COUNT(*) AS `likes`",
            "FROM `likes`",
            "GROUP BY `message_id`",
            ")",
            "AS `X`",
            "ON `messages`.`message_id`=`X`.`message_id`",
            "ORDER BY created_at DESC"
        ].join(" "),
        vec![user_id.into()]
    ))
    .all(db)
    .await
}

#[derive(FromQueryResult)]
pub struct Message {
    pub message_id: String,
    pub user_name: String,
    pub message_text: String,
    pub created_at: DateTimeWithTimeZone,
    pub likes: i64,
    pub is_like: i64,
}

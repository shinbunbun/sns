use actix_session::Session;
use sea_orm::DatabaseConnection;

use crate::usecase;

const SESSION_KEY: &str = "user";

pub async fn is_valid(db: &DatabaseConnection, session: &Session) -> bool {
    let user_session = match session.get::<String>(SESSION_KEY) {
        Ok(res) => res,
        Err(_) => return false,
    };
    let user_id = match user_session {
        Some(x) => x,
        None => return false,
    };
    let select_result = usecase::user::select_by_user_id(db, &user_id).await;
    if select_result.is_err() {
        return false;
    };
    true
}

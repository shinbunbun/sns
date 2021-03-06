use actix_session::Session;
use actix_web::error;
use sea_orm::DatabaseConnection;

use crate::usecase;

const SESSION_KEY: &str = "user";

pub async fn get_user(db: &DatabaseConnection, session: &Session) -> Option<entity::user::Model> {
    let user_session = match session.get::<String>(SESSION_KEY) {
        Ok(res) => res,
        Err(_) => return None,
    };
    let user_id = match user_session {
        Some(x) => x,
        None => return None,
    };
    let select_result = usecase::user::select_by_user_id(db, &user_id).await;
    match select_result {
        Ok(res) => res,
        Err(_) => None,
    }
}

pub fn insert(session: &Session, value: &str) -> Result<(), error::Error> {
    session.insert(SESSION_KEY, value)
}

pub fn remove(session: &Session) -> Option<String> {
    session.remove(SESSION_KEY)
}

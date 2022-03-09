use sea_orm::DbErr;

#[derive(Debug)]
pub enum AppError {
    Internal(String),
    Validation(validator::ValidationErrors),
    Db(DbErr),
    Password(String),
    Session(actix_web::Error),
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppError::Validation(_)
            | AppError::Db(_)
            | AppError::Password(_)
            | AppError::Session(_)
            | AppError::Internal(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        actix_web::HttpResponse::build(self.status_code())
            .insert_header(actix_web::http::header::ContentType::html())
            // TODO: エラーページを返してあげる
            .body(self.to_string()) // Displayを実装したから to_string() メソッドが使える
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Validation(errors) => write!(f, "Validation Error: {errors}"),
            AppError::Db(error) => write!(f, "Db Error: {error}"),
            AppError::Internal(error) => write!(f, "Internal Server Error: {error}"),
            AppError::Password(error) => write!(f, "{error}"),
            AppError::Session(error) => write!(f, "Session Error {error}"),
        }
    }
}

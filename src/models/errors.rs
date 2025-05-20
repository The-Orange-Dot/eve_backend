use actix_web::{HttpResponse};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
  Io(std::io::Error),
  Sqlx(sqlx::Error),
  Env(std::env::VarError),
  Actix(actix_web::Error)
}

 impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      AppError::Io(err) => write!(f, "IO Error: {}", err),
      AppError::Sqlx(err) => write!(f, "Database Error: {}", err),
      AppError::Env(err) => write!(f, "Environment Error: {}", err),
      AppError::Actix(err) => write!(f, "Actix Error: {}", err)
    }
  }
}

// Implement ResponseError for Actix Web compatibility
impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Io(_) => HttpResponse::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
            AppError::Sqlx(_) => HttpResponse::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
            AppError::Env(_) => HttpResponse::new(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
            AppError::Actix(err) => err.error_response(),
        }
    }
}

// From implementations for error conversion
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Sqlx(err)
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        AppError::Env(err)
    }
}

impl From<actix_web::Error> for AppError {
    fn from(err: actix_web::Error) -> Self {
        AppError::Actix(err)
    }
}
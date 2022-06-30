pub mod assign;
pub mod task;
pub mod user;

use actix_web::{
  body::BoxBody,
  http::{header::ContentType, StatusCode},
  HttpRequest, HttpResponse, Responder,
};

use actix_web::error::ResponseError;

#[derive(Debug)]
enum MyError {
  InternalServerError,
  InvalidFormat,
  NotFound,
}

impl std::fmt::Display for MyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match *self {
        MyError::InternalServerError => "Interal Server Error",
        MyError::InvalidFormat => "Invalid Format",
        MyError::NotFound => "Not Found",
      }
    )?;

    Ok(())
  }
}

impl ResponseError for MyError {
  fn error_response(&self) -> HttpResponse<BoxBody> {
    HttpResponse::build(self.status_code())
      .insert_header(ContentType::html())
      .body(self.to_string())
  }

  fn status_code(&self) -> actix_web::http::StatusCode {
    match *self {
      MyError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
      MyError::InvalidFormat => StatusCode::BAD_REQUEST,
      MyError::NotFound => StatusCode::NOT_FOUND,
    }
  }
}

pub struct Json(String);

impl Responder for Json {
  type Body = BoxBody;

  fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
    HttpResponse::Ok()
      .content_type(ContentType::json())
      .body(self.0)
  }
}

fn convert_error<T>(_err: T) -> MyError {
  MyError::InternalServerError
}

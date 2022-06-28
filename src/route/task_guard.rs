// data guard for task

use super::super::model::task::{NewTask, UpdateTask};

use rocket::http::Status;

use rocket::data::{self, ByteUnit, Data, FromData};
use rocket::request::Request;

#[rocket::async_trait]
impl<'r> FromData<'r> for NewTask {
  type Error = &'r str;

  async fn from_data(
    _req: &'r Request<'_>,
    data: Data<'r>,
  ) -> data::Outcome<'r, Self, Self::Error> {
    use rocket::outcome::Outcome::{Failure, Success};

    let body = match data.open(ByteUnit::KiB).into_string().await {
      Ok(body) if body.is_complete() => body.into_inner(),
      Ok(_) => return Failure((Status::PayloadTooLarge, "Too large")),
      Err(_) => return Failure((Status::InternalServerError, "Internal Error")),
    };

    match serde_json::from_str(body.as_str()) {
      Ok(v) => Success(v),
      Err(_) => Failure((Status::NotAcceptable, "Invalid format")),
    }
  }
}

#[rocket::async_trait]
impl<'r> FromData<'r> for UpdateTask {
  type Error = &'r str;

  async fn from_data(
    _req: &'r Request<'_>,
    data: Data<'r>,
  ) -> data::Outcome<'r, Self, Self::Error> {
    use rocket::outcome::Outcome::{Failure, Success};

    let body = match data.open(ByteUnit::KiB).into_string().await {
      Ok(body) if body.is_complete() => body.into_inner(),
      Ok(_) => return Failure((Status::PayloadTooLarge, "Too large")),
      Err(_) => return Failure((Status::InternalServerError, "Internal Error")),
    };

    println!("{}", body);

    match serde_json::from_str(body.as_str()) {
      Ok(v) => Success(v),
      Err(_) => Failure((Status::NotAcceptable, "Invalid format")),
    }
  }
}

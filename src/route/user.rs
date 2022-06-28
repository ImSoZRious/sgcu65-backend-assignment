use super::super::db::DbConn;
use super::super::model::user::{NewUser, UpdateUser, User};

use super::Json;

use rocket::http::Status;

use rocket::data::{self, ByteUnit, Data, FromData};
use rocket::request::Request;

#[rocket::async_trait]
impl<'r> FromData<'r> for NewUser {
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
impl<'r> FromData<'r> for UpdateUser {
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

#[get("/")]
pub async fn get(conn: DbConn) -> Result<Json, Status> {
  let result = User::get_all(&conn);

  match result {
    Ok(users) => match serde_json::to_string(&users) {
      Ok(user_string) => Ok(Json(user_string)),
      Err(_) => Err(Status::InternalServerError),
    },
    Err(_) => Err(Status::InternalServerError),
  }
}

#[post("/", format = "application/json", data = "<user>")]
pub async fn create(user: NewUser, conn: DbConn) -> Result<Json, Status> {
  let result = User::create(&user, &conn);

  match result {
    Ok(users) => match serde_json::to_string(&users) {
      Ok(user_string) => Ok(Json(user_string)),
      Err(_) => Err(Status::InternalServerError),
    },
    Err(_) => Err(Status::InternalServerError),
  }
}

#[put("/<id>", format = "application/json", data = "<user>")]
pub async fn update(mut user: UpdateUser, conn: DbConn, id: i32) -> Status {
  user.id = id;

  let result = User::update(&user, &conn);

  match result {
    Ok(_) => Status::Ok,
    Err(_) => Status::InternalServerError,
  }
}

#[delete("/<id>")]
pub async fn delete(id: i32, conn: DbConn) -> Status {
  let result = User::delete(id, &conn);

  match result {
    Ok(1) => Status::Ok,
    Ok(_) => Status::NotFound,
    Err(_) => Status::InternalServerError,
  }
}

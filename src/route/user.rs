use super::super::db::DbConn;
use super::super::model::user::{NewUser, UpdateUser, User};

use super::Json;

use rocket::http::Status;

#[get("/", rank = 2)]
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

#[get("/<id>", rank = 1)]
pub async fn find(id: i32, conn: DbConn) -> Result<Json, Status> {
  let result = User::find(id, &conn);

  match result {
    Ok(user) => match serde_json::to_string(&user) {
      Ok(user_string) => Ok(Json(user_string)),
      Err(_) => Err(Status::InternalServerError),
    },
    Err(_) => Err(Status::InternalServerError),
  }
}

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod db;
mod model;
mod route;
mod schema;

#[launch]
fn rocket() -> _ {
  rocket::build().manage(db::init_pool()).mount(
    "/user",
    routes![
      route::user::get,
      route::user::create,
      route::user::update,
      route::user::delete
    ],
  )
}

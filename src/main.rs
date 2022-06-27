#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod model;
mod schema;

#[get("/")]
fn hello() -> &'static str {
  "Hello, world!"
}

#[launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![hello])
}

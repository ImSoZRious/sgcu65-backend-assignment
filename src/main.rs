#[macro_use]
extern crate diesel;

// mod db;
mod model;
// mod route;
mod schema;

// use actix_web::{web, App, HttpServer};
// use db::init_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  Ok(())
  // let db_pool = init_pool();

  // HttpServer::new(move || {
  //   let user_scope = route::user::get_scope();
  //   let task_scope = route::task::get_scope();
  //   let assign_scope = route::assign::get_scope();
  //   App::new()
  //     .app_data(web::Data::new(db_pool.clone()))
  //     .service(user_scope)
  //     .service(task_scope)
  //     .service(assign_scope)
  // })
  // .bind(("127.0.0.1", 8080))?
  // .run()
  // .await
}

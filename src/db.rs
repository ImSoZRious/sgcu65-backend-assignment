use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use rocket::async_trait;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome};
use rocket::{Request, State};
use std::ops::Deref;

fn get_db_url() -> String {
  use dotenv::dotenv;
  use std::env;
  dotenv().ok();

  env::var("DATABASE_URL")
    .expect("No database env is set.")
    .to_string()
}

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type Connection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
  let db_url = get_db_url();
  println!("Found url: {}", db_url);
  let manager = ConnectionManager::<PgConnection>::new(db_url);
  match Pool::new(manager) {
    Ok(n) => n,
    Err(_) => {
      panic!("DB Pool")
    }
  }
}

pub struct DbConn(pub Connection);

#[async_trait]
impl<'r> FromRequest<'r> for DbConn {
  type Error = ();

  async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
    let pool = request.guard::<&State<Pool>>().await.unwrap();

    match pool.get() {
      Ok(conn) => Outcome::Success(DbConn(conn)),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
    }
  }
}

impl Deref for DbConn {
  type Target = Connection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

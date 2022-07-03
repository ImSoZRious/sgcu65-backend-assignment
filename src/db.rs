use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::ops::Deref;

fn get_db_url() -> String {
  use dotenv::dotenv;
  use std::env;

  match env::var("DATABASE_URL") {
    Ok(v) => v.to_string(),
    Err(_) => {
      // load .env if no url is found
      dotenv().ok();

      env::var("DATABASE_URL")
        .expect("No database env is set.")
        .to_string()
    }
  }
}

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type Connection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
  let db_url = get_db_url();
  println!("[Init db]: Found url: {}", db_url);
  let manager = ConnectionManager::<PgConnection>::new(db_url);
  println!("[Init db]: Created connection manager");
  match Pool::new(manager) {
    Ok(n) => n,
    Err(_) => {
      panic!(
        "There's an error occured when creating connection pool, please recheck your database URL."
      )
    }
  }
}

pub struct DbConn(pub Connection);

impl Deref for DbConn {
  type Target = Connection;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

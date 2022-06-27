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

#[cfg(test)]
mod tests {

    use super::model::User;

    use diesel::pg::PgConnection;
    use diesel::prelude::*;

    #[test]
    fn test() {
        let database_url = "postgres://postgres:trust@localhost:12345/isd";
        let conn = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        let users = User::get_all_users(&conn);
        println!("{:?}", users);
    }
}

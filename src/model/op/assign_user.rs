use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use super::super::team::Team;
use super::super::user::User;
use crate::schema::teams;
use crate::schema::teams::dsl::teams as all_teams;
use crate::schema::users;
use crate::schema::users::dsl::users as all_users;
use diesel::prelude::*;
use diesel::result::Error;

impl User {
  pub fn join_team(user_id: i32, team_id: i32, conn: &PgConnection) -> Result<(), Error> {
    let row = diesel::update(all_users.filter(users::id.eq(user_id)))
      .set(users::team_id.eq(team_id))
      .execute(conn);

    match row {
      Ok(1) => Ok(()),
      Ok(_) => Err(diesel::result::Error::NotFound),
      Err(n) => Err(n),
    }
  }

  pub fn get_team(&self, conn: &PgConnection) -> Result<Option<Team>, Error> {
    if self.team_id.is_none() {
      return Ok(None);
    }
    let team_id = self.team_id.unwrap();
    let result = all_teams.filter(teams::id.eq(team_id)).load::<Team>(conn)?;

    Ok(Some(result[0].clone()))
  }
}

impl Team {
  pub fn assign_user(user_id: i32, team_id: i32, conn: &PgConnection) -> Result<(), Error> {
    let row = diesel::update(all_users.filter(users::id.eq(user_id)))
      .set(users::team_id.eq(team_id))
      .execute(conn);

    match row {
      Ok(1) => Ok(()),
      Ok(_) => Err(diesel::result::Error::NotFound),
      Err(n) => Err(n),
    }
  }

  pub fn get_user(&self, conn: &PgConnection) -> Result<Vec<User>, Error> {
    let team_id = self.id;

    let result = all_users
      .filter(users::team_id.eq(team_id))
      .load::<User>(conn)?;

    Ok(result)
  }
}

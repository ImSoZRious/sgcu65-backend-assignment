use crate::schema::teams;
use crate::schema::teams::dsl::teams as all_teams;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, QueryableByName, Clone, Serialize)]
#[table_name = "teams"]
pub struct Team {
  pub id: i32,
  pub name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "teams"]
pub struct NewTeam {
  pub name: String,
}

#[derive(Deserialize, Default)]
pub struct PartialTeam {
  pub id: Option<i32>,
  pub name: Option<String>,
}

impl Team {
  pub fn create(team: &NewTeam, conn: &PgConnection) -> Result<Team, Error> {
    diesel::insert_into(teams::table)
      .values(team)
      .get_results(conn)
      .map(|team_vec: Vec<Team>| team_vec[0].clone())
  }

  pub fn get_all(conn: &PgConnection) -> Result<Vec<Team>, Error> {
    all_teams.order(teams::id.desc()).load::<Team>(conn)
  }

  pub fn update(team: &PartialTeam, conn: &PgConnection) -> Result<(), Error> {
    let set_string = team.set_string();

    if team.no_id() {
      panic!("No id is supplied");
    }

    if team.all_none() {
      panic!("all other field are None")
    }

    let query_string: String = format!(
      "UPDATE teams \
        SET {} \
        WHERE id = {}",
      set_string,
      team.id.unwrap()
    );

    // Result is not actually team since this query isn't select query
    let res: Result<Vec<Team>, Error> = diesel::sql_query(query_string).load(conn);

    match res {
      Ok(_) => Ok(()),
      Err(e) => Err(e),
    }
  }

  pub fn delete(team_id: i32, conn: &PgConnection) -> Result<usize, Error> {
    diesel::delete(all_teams)
      .filter(teams::id.eq(team_id))
      .execute(conn)
  }

  pub fn find(team_id: i32, conn: &PgConnection) -> Result<Team, Error> {
    all_teams.find(team_id).first(conn)
  }

  pub fn query(query: &PartialTeam, conn: &PgConnection) -> Result<Vec<Team>, Error> {
    let filter_string = query.like_string();

    let query_string = format!(
      "SELECT * \
      FROM teams \
      WHERE {}",
      filter_string
    );

    diesel::sql_query(query_string).load(conn)
  }

  pub fn query_id(ids: Vec<i32>, conn: &PgConnection) -> Result<Vec<Team>, Error> {
    all_teams.filter(teams::id.eq_any(ids)).load(conn)
  }
}

impl PartialTeam {
  pub fn all_none(&self) -> bool {
    self.name.is_none()
  }

  pub fn no_id(&self) -> bool {
    self.id.is_none()
  }

  fn join(&self, op: impl ToString, sep: impl ToString) -> String {
    let mut string_vec: Vec<String> = vec![];
    let op = op.to_string();

    if let Some(name) = &self.name {
      string_vec.push(format!("name {} '{}'", op, name));
    }

    string_vec.join(sep.to_string().as_str())
  }

  pub fn set_string(&self) -> String {
    self.join("=", ",")
  }

  pub fn filter_string(&self) -> String {
    self.join("=", " AND ")
  }

  pub fn like_string(&self) -> String {
    let mut string_vec: Vec<String> = vec![];

    if let Some(name) = &self.name {
      string_vec.push(format!("name LIKE '{}%'", name));
    }

    string_vec.join(" AND ")
  }
}

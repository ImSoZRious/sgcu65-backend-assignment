use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use super::super::task::Task;
use super::super::team::Team;
use crate::schema::tasks;
use crate::schema::tasks::dsl::tasks as all_tasks;
use crate::schema::teams;
use crate::schema::teams::dsl::teams as all_teams;
use diesel::prelude::*;
use diesel::result::Error;

impl Task {
  pub fn assign_to(task_id: i32, team_id: i32, conn: &PgConnection) -> Result<(), Error> {
    let row = diesel::update(all_tasks.filter(tasks::id.eq(task_id)))
      .set(tasks::owner_team_id.eq(team_id))
      .execute(conn);

    match row {
      Ok(1) => Ok(()),
      Ok(_) => Err(diesel::result::Error::NotFound),
      Err(n) => Err(n),
    }
  }

  pub fn get_team(&self, conn: &PgConnection) -> Result<Option<Team>, Error> {
    if self.owner_team_id.is_none() {
      return Ok(None);
    }
    let team_id = self.owner_team_id.unwrap();
    let result = all_teams.filter(teams::id.eq(team_id)).load::<Team>(conn)?;

    Ok(Some(result[0].clone()))
  }
}

impl Team {
  pub fn accept_task(task_id: i32, team_id: i32, conn: &PgConnection) -> Result<(), Error> {
    let row = diesel::update(all_tasks.filter(tasks::id.eq(task_id)))
      .set(tasks::owner_team_id.eq(team_id))
      .execute(conn);

    match row {
      Ok(1) => Ok(()),
      Ok(_) => Err(diesel::result::Error::NotFound),
      Err(n) => Err(n),
    }
  }

  pub fn get_task(&self, conn: &PgConnection) -> Result<Vec<Task>, Error> {
    let team_id = self.id;

    let result = all_tasks
      .filter(tasks::owner_team_id.eq(team_id))
      .load::<Task>(conn)?;

    Ok(result)
  }
}

use crate::schema::teams_tasks;
use crate::schema::teams_tasks::dsl::teams_tasks as all_tt;
use diesel::prelude::*;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use diesel::PgConnection;

use super::task::Task;
use super::team::Team;

use diesel::result::Error;

#[derive(Clone, Serialize, Insertable, Queryable, Debug, Deserialize)]
#[table_name = "teams_tasks"]
pub struct TeamsTasks {
  pub team_id: i32,
  pub task_id: i32,
}

impl TeamsTasks {
  pub fn create(assign: &TeamsTasks, conn: &PgConnection) -> Result<TeamsTasks, Error> {
    diesel::insert_into(teams_tasks::table)
      .values(assign)
      .get_results(conn)
      .map(|assign: Vec<TeamsTasks>| assign[0].clone())
  }

  pub fn from_team(team_id: i32, conn: &PgConnection) -> Result<Vec<Task>, Error> {
    let res: Result<Vec<TeamsTasks>, Error> = all_tt
      .select((teams_tasks::team_id, teams_tasks::task_id))
      .filter(teams_tasks::team_id.eq_all(team_id))
      .load(conn);

    res.and_then(|tts: Vec<TeamsTasks>| {
      Task::query_id(tts.into_iter().map(|tt| tt.task_id).collect(), conn)
    })
  }

  pub fn from_task(task_id: i32, conn: &PgConnection) -> Result<Vec<Team>, Error> {
    let res: Result<Vec<TeamsTasks>, Error> = all_tt
      .select((teams_tasks::team_id, teams_tasks::task_id))
      .filter(teams_tasks::task_id.eq_all(task_id))
      .load(conn);

    res.and_then(|tts: Vec<TeamsTasks>| {
      Team::query_id(tts.into_iter().map(|tt| tt.team_id).collect(), conn)
    })
  }
}

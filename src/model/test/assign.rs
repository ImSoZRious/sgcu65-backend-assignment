use super::super::assign::TeamsTasks;
use super::super::task::Task;
use super::super::team::Team;
use super::get_db_con;
use super::s;

#[test]
fn assign_task() {
  let conn = get_db_con();
  let team_id = Team::get_all(&conn).unwrap()[0].id;
  let task_id = Task::get_all(&conn).unwrap()[0].id;

  let assign = TeamsTasks {
    team_id: team_id,
    task_id: task_id,
  };

  let assign = TeamsTasks::create(&assign, &conn).unwrap();

  println!("{:?}", assign);
}

#[test]
fn get_task_from_team() {
  let conn = get_db_con();

  let team_id = Team::get_all(&conn).unwrap()[0].id;
  let _tasks = TeamsTasks::from_team(team_id, &conn);

  println!("{:?}", _tasks);
}

#[test]
fn get_team_from_task() {
  let conn = get_db_con();

  let task_id = Task::get_all(&conn).unwrap()[0].id;
  let _teams = TeamsTasks::from_task(task_id, &conn);

  println!("{:?}", _teams);
}

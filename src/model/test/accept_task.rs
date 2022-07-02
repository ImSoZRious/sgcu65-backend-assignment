use super::super::task::Task;
use super::super::team::Team;
use super::get_db_con;
use super::s;
#[test]
fn accept_task() {
  let conn = get_db_con();
  let team_id = Team::get_all(&conn).unwrap()[0].id;
  let task_id = Task::get_all(&conn).unwrap()[0].id;

  Task::assign_to(task_id, team_id, &conn).unwrap();

  assert_eq!(
    Task::find(task_id, &conn).unwrap().owner_team_id.unwrap(),
    team_id
  );
}

#[test]
fn get_team_form_task() {
  let conn = get_db_con();

  let task = &Task::get_all(&conn)
    .unwrap()
    .into_iter()
    .filter(|task| task.owner_team_id.is_some())
    .collect::<Vec<Task>>()[0];

  let team = task.get_team(&conn).unwrap().unwrap();

  println!("{:?}", team);
}

#[test]
fn get_task_from_team() {
  let conn = get_db_con();
  let team = Team::find(10007, &conn).unwrap();

  let task = team.get_task(&conn);

  println!("{:?}", task);
}

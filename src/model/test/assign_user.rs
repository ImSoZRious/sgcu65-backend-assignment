use super::super::team::Team;
use super::super::user::User;
use super::get_db_con;

#[test]
fn assign_user() {
  let conn = get_db_con();
  let team_id = Team::get_all(&conn).unwrap()[0].id;
  let user_id = User::get_all(&conn).unwrap()[0].id;

  User::join_team(user_id, team_id, &conn).unwrap();

  assert_eq!(
    User::find(user_id, &conn).unwrap().team_id.unwrap(),
    team_id
  );
}

#[test]
fn get_team_form_user() {
  let conn = get_db_con();

  let user = &User::get_all(&conn)
    .unwrap()
    .into_iter()
    .filter(|user| user.team_id.is_some())
    .collect::<Vec<User>>()[0];

  let team = user.get_team(&conn).unwrap().unwrap();

  println!("{:?}", team);
}

#[test]
fn get_user_from_team() {
  let conn = get_db_con();
  let team = Team::find(10007, &conn).unwrap();

  let user = team.get_user(&conn);

  println!("{:?}", user);
}

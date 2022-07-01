use super::super::team::{NewTeam, PartialTeam, Team};
use super::get_db_con;
use super::s;

#[test]
fn get_all_team() {
  let conn = get_db_con();

  let _teams = Team::get_all(&conn).unwrap();

  println!("{:?}", _teams);
}

#[test]
fn create_team() {
  let conn = get_db_con();

  let new_team = NewTeam {
    name: s!("Let's game it out"),
  };

  let _team = Team::create(&new_team, &conn).unwrap();

  println!("{:?}", _team);
}

#[test]
fn update_team() {
  let conn = get_db_con();

  let update_info = PartialTeam {
    id: Some(10001),
    name: Some(s!("Calculus")),
    ..Default::default()
  };

  let _result = Team::update(&update_info, &conn).unwrap();
}

#[test]
fn delete_team() {
  let conn = get_db_con();

  let delete_id = 10001;

  let _del_count = Team::delete(delete_id, &conn).unwrap();
}

#[test]
fn find_team() {
  let conn = get_db_con();

  let team_id = 10002;

  let _team = Team::find(team_id, &conn).unwrap();

  println!("{:?}", _team);
}

#[test]
fn find_team_by_name() {
  let conn = get_db_con();
  let team_name = "Let";

  let q = PartialTeam {
    name: Some(s!(team_name)),
    ..Default::default()
  };

  let _team = Team::query(&q, &conn).unwrap();

  println!("{:?}", _team);
}

#[test]
fn combined_team_test() {
  let conn = get_db_con();

  let users = vec![
    NewTeam { name: s!("Nono") },
    NewTeam {
      name: s!("Im tired"),
    },
    NewTeam {
      name: s!("IOI Boids"),
    },
  ];

  for team in users.iter() {
    let _team = Team::create(team, &conn).unwrap();
  }

  let edit_infos = vec![PartialTeam {
    id: Some(10003),
    name: Some(s!("yay")),
  }];

  for edit_info in edit_infos.iter() {
    let _ = Team::update(edit_info, &conn).unwrap();
  }

  let users = Team::get_all(&conn).unwrap();
  let first = users.len();

  let _del = Team::delete(10003, &conn);
  let users = Team::get_all(&conn).unwrap();
  assert_eq!(users.len(), first - 1);
}

table! {
    session (id) {
        id -> Int4,
        api_key -> Varchar,
        expire_time -> Varchar,
        user_id -> Int4,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        name -> Varchar,
        content -> Varchar,
        status -> Varchar,
        deadline -> Varchar,
        owner_team_id -> Nullable<Int4>,
    }
}

table! {
    teams (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        role -> Varchar,
        team_id -> Nullable<Int4>,
        pwd_hash -> Varchar,
        permission -> Varchar,
    }
}

joinable!(session -> users (user_id));
joinable!(users -> teams (team_id));

allow_tables_to_appear_in_same_query!(
    session,
    tasks,
    teams,
    users,
);

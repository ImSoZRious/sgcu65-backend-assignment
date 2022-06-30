table! {
    tasks (id) {
        id -> Int4,
        name -> Varchar,
        content -> Text,
        status -> Varchar,
        deadline -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        role -> Varchar,
    }
}

table! {
    users_tasks (user_id, task_id) {
        user_id -> Int4,
        task_id -> Int4,
    }
}

joinable!(users_tasks -> tasks (task_id));
joinable!(users_tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
    users_tasks,
);

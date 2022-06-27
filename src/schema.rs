table! {
    tasks (id) {
        id -> Int4,
        name -> Varchar,
        content -> Text,
        status -> Varchar,
        timestamp -> Varchar,
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

allow_tables_to_appear_in_same_query!(tasks, users,);

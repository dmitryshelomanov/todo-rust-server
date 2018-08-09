table! {
    sessions (id) {
        id -> Integer,
        token -> Varchar,
        user_id -> Integer,
    }
}

table! {
    todos (id) {
        id -> Integer,
        title -> Varchar,
        checked -> Bool,
    }
}

table! {
    users (id) {
        id -> Integer,
        login -> Varchar,
        password -> Varchar,
    }
}

joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    sessions,
    todos,
    users,
);

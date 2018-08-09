table! {
    sessions (id) {
        id -> Integer,
        token -> Nullable<Varchar>,
        user_id -> Nullable<Integer>,
    }
}

table! {
    todos (id) {
        id -> Integer,
        title -> Nullable<Varchar>,
        checked -> Nullable<Bool>,
    }
}

table! {
    users (id) {
        id -> Integer,
        login -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
    }
}

joinable!(sessions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    sessions,
    todos,
    users,
);

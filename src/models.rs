use schema::{todos, users};

#[derive(Serialize, Queryable, Clone, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub user_id: i32,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "todos"]
pub struct AsChangesetTodo {
    pub title: Option<String>,
    pub checked: Option<bool>,
}

#[derive(Insertable, Deserialize)]
#[table_name = "todos"]
pub struct InsertableTodo {
    pub title: String,
    pub checked: bool,
    pub user_id: i32,
}

#[derive(Serialize, Clone)]
pub struct Session {
    pub user_id: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct InsertableUser {
    pub login: String,
    pub password: String,
}

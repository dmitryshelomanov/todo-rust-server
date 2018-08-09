use schema::todos;

#[derive(Serialize, Queryable, Clone, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub checked: bool,
}

#[derive(Insertable)]
#[table_name="todos"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub checked: &'a bool,
}

#[derive(AsChangeset, Deserialize)]
#[table_name="todos"]
pub struct UpdatedTodo {
    pub title: Option<String>,
    pub checked: Option<bool>,
}

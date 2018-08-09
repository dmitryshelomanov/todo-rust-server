use schema::todos;

#[derive(Serialize, Queryable, Clone, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub checked: bool,
}

#[derive(Insertable, AsChangeset, Deserialize)]
#[table_name = "todos"]
pub struct FormTodo {
    pub title: Option<String>,
    pub checked: Option<bool>,
}

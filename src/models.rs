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

impl<'a> NewTodo<'a> {
    pub fn new(title: &'a str) -> NewTodo<'a> {
        NewTodo {
            title,
            checked: &false,
        }
    }
}

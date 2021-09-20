use super::schema::todos;
use chrono::{self, NaiveDateTime};

#[derive(Insertable, Debug)]
#[table_name = "todos"]
pub struct NewTodo {
    pub name: String,
    pub detail: Option<String>,
    pub date_from: Option<NaiveDateTime>,
    pub date_to: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct Todo {
    id: i64,
    name: String,
    detail: Option<String>,
    date_from: Option<NaiveDateTime>,
    date_to: Option<NaiveDateTime>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="todo")]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub checked: bool
}

#[derive(Serialize, Deserialize)]
pub struct CreateTodo {
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub success: bool
}
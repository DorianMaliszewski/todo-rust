use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

use deadpool_postgres::Client;
use crate::models::Todo;

pub async fn get_todos(client: &Client) -> Result<Vec<Todo>, io::Error> {
    let statement = client.prepare("Select * from todo order by id desc").await.unwrap();

    let todos = client.query(&statement, &[])
    .await.expect("Error when getting todo")
    .iter()
    .map(|row| Todo::from_row_ref(row).unwrap())
    .collect::<Vec<Todo>>();

    Ok(todos)
}

pub async fn get_todo(client: &Client, todo_id: i32) -> Result<Todo, io::Error> {
    let statement = client.prepare("Select * from todo where id = $1").await.unwrap();

    client.query(&statement, &[&todo_id])
    .await
    .expect("Error when getting todo")
    .iter()
    .map(|row| Todo::from_row_ref(row).unwrap())
    .collect::<Vec<Todo>>()
    .pop()
    .ok_or(io::Error::new(io::ErrorKind::Other, "Error when getting the todo"))

}

pub async fn create_todo(client: &Client, title: String) -> Result<Todo, io::Error> {
    let statement = client.prepare("INSERT INTO todo (title) VALUES ($1) return id, title, checked").await.unwrap();

    client.query(&statement, &[&title])
    .await
    .expect("Error when creating todo")
    .iter()
    .map(|row| Todo::from_row_ref(row).unwrap())
    .collect::<Vec<Todo>>()
    .pop()
    .ok_or(io::Error::new(io::ErrorKind::Other, "Error when creating todo"))

}


pub async fn check_todo(client: &Client, todo_id: i32) -> Result<(), io::Error> {
    let statement = client.prepare("Update todo set checked = true where id = $1 and checked = false").await.unwrap();

    let result = client.execute(&statement, &[&todo_id])
    .await
    .expect("Error when creating todo");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Can't check the todo"))
    }

}
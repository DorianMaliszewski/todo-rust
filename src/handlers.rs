
use std::io::ErrorKind::Other;

use crate::models::{CreateTodo, Result};
use deadpool_postgres::Pool;
use deadpool_postgres::Client;
use actix_web::Responder;
use actix_web::HttpResponse;
use actix_web::web;


use crate::models::Status;
use crate::db;

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "OK".to_string()})
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error when connecting to the dtaabse");

    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn get_todo(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error when connecting to the dtaabse");

    let result = db::get_todo(&client, path.0).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}


pub async fn create_todo(db_pool: web::Data<Pool>, json: web::Json<CreateTodo>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error when connecting to the dtaabse");

    let result = db::create_todo(&client, json.title.clone()).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn check_todo(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error when connecting to the dtaabse");

    let result = db::check_todo(&client, path.0).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(Result{success: true}),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(Result{success: false}),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}
use crate::application::todo_service::TodoService;
use crate::domain::todo::{CreateTodo, UpdateTodo};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_todos)
        .service(get_todo)
        .service(create_todo)
        .service(update_todo)
        .service(delete_todo);
}

#[get("/todos")]
async fn get_todos(service: web::Data<TodoService>) -> impl Responder {
    let todos = service.get_all();
    HttpResponse::Ok().json(todos)
}

#[get("/todos/{id}")]
async fn get_todo(service: web::Data<TodoService>, path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    if let Some(todo) = service.get_by_id(id) {
        HttpResponse::Ok().json(todo)
    } else {
        HttpResponse::NotFound().json(json!({"error": "Todo not found"}))
    }
}

#[post("/todos")]
async fn create_todo(
    service: web::Data<TodoService>,
    body: web::Json<CreateTodo>,
) -> impl Responder {
    let new_todo = service.create(body.into_inner());
    HttpResponse::Created().json(new_todo)
}

#[patch("/todos/{id}")]
async fn update_todo(
    service: web::Data<TodoService>,
    path: web::Path<u64>,
    body: web::Json<UpdateTodo>,
) -> impl Responder {
    let id = path.into_inner();
    if let Some(updated_todo) = service.update(id, body.into_inner()) {
        HttpResponse::Ok().json(updated_todo)
    } else {
        HttpResponse::NotFound().json(json!({"error": "Todo not found"}))
    }
}

#[delete("/todos/{id}")]
async fn delete_todo(service: web::Data<TodoService>, path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner();
    if service.delete(id) {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().json(json!({"error": "Todo not found"}))
    }
}
use actix_web::{web, App, HttpServer};
use rust_todo_api::application::todo_service::TodoService;
use rust_todo_api::infrastructure::web::configure_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_service = web::Data::new(TodoService::new());

    HttpServer::new(move || {
        App::new()
            .app_data(todo_service.clone())
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

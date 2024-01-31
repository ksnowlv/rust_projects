use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::handlers::user_handler::user_routes;

mod handlers;
mod models;
mod db;
mod cache;
mod config;
mod middleware;


async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix-web!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(user_routes).route("", web::get().to(index))

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
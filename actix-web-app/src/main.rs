use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use crate::handlers::user_handler::user_routes;
use actix_web::web::{ scope};
mod handlers;
mod models;
mod db;
mod cache;
mod config;
mod middleware;
mod actix_swagger;


use swagger_ui;
use actix_swagger::lib::swagger;

async fn index(path: web::Path<(i32,)>) -> impl Responder {
    let value = path.0;
    HttpResponse::Ok().body(format!("Hello, Actix-web! The value is {}", value))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let http_server = HttpServer::new(|| {
        let spec = swagger_ui::swagger_spec_file!("actix_swagger/openapi.json");
        let config = swagger_ui::Config::default();

        App::new().service(scope("/api/v1/swagger")
                .configure(swagger(spec, config)))
            .configure(user_routes).route("/{value}", web::get().to(index))
    });

    http_server.bind("127.0.0.1:8084")?
    .run()
    .await
}

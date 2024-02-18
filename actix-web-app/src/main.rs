use actix_web::{HttpRequest, post};
use actix_http::error::PayloadError;
use actix_http::KeepAlive;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware};
use actix_web::dev::ServiceRequest;
use actix_web::Error;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_web::middleware::Compress;
use actix_web::web::{ scope};
use swagger_ui;
use actix_swagger::lib::swagger;
use crate::middlewares::{logger::init_logger, logging_middleware, auth_middleware, heartbeat_middleware };
use serde::Serialize;
use serde_derive::Deserialize;
use crate::db::db::{initialize_pool, get_pool };
use crate::handlers::file_handler::file_routes;
use crate::handlers::user_handler::user_routes;

mod handlers;
mod models;
mod db;
mod cache;
mod config;
mod middlewares;
mod actix_swagger;

#[derive(Debug, Serialize, Deserialize)]
struct MyData {
    code: i32,
    message: String,
}

#[post("/from_json")]
async fn from_json(my_data: web::Json<MyData>) -> impl Responder {
    println!("Received mydata from JSON: {:?}", my_data);
    HttpResponse::Ok().json(my_data)
}

#[post("/to_json")]
async fn to_json() -> impl Responder {
    let my_data = MyData { code: 1, message: "OK".to_string() };
    let json_data = serde_json::to_string(&my_data).unwrap();
    HttpResponse::Ok().body(json_data)
}

async fn index(path: web::Path<(i32,)>) -> impl Responder {
    let value = path.0;

    let response_data = MyData { code:200, message: "Hello, world!".to_string() };
    HttpResponse::Ok().json(response_data)
}

// 自定义错误处理程序函数
fn handle_json_payload_error(err: actix_web::error::JsonPayloadError, _req: &HttpRequest) -> Error {
    // 在这里处理 JSON payload 错误，例如返回适当的错误响应或记录错误日志
    println!("handle_json_payload_error:{:?}", err);
    err.into()
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    initialize_pool().await;

    let http_server = HttpServer::new(|| {
        let spec = swagger_ui::swagger_spec_file!("actix_swagger/openapi.json");
        let config = swagger_ui::Config::default();

        App::new()//.service(scope("/api/v1/swagger")
//                .configure(swagger(spec, config)))
            .wrap(middleware::Logger::default())
//           //.wrap(auth_middleware::Auth)
            .wrap(logging_middleware::Logging)
           // .wrap(heartbeat_middleware::Heartbeat)
           .wrap(Compress::default())
            .app_data(web::Data::new(get_pool()))
            //全局JSON负载的最大大小为2MB,并配置自定义错误处理函数
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 2).error_handler(handle_json_payload_error))
            .configure(user_routes).configure(file_routes).service(to_json).service(from_json)
    });

    // 创建 SSL 加密器
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file("key.pem", SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    http_server//.bind("127.0.0.1:8080")?
        .bind_openssl("127.0.0.1:8080", builder)?
        .keep_alive(KeepAlive::default())
    .run()
    .await
}

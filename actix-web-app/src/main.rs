use actix_http::error::PayloadError;
use actix_http::KeepAlive;
use actix_web::{web, App, HttpServer, Responder, HttpResponse, middleware};
use actix_web::cookie::time::format_description::FormatItem::Component;
use actix_web::dev::ServiceRequest;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::handlers::user_handler::user_routes;
use actix_web::middleware::Compress;
use actix_web::web::{ scope};

mod handlers;
mod models;
mod db;
mod cache;
mod config;
mod middlewares;
mod actix_swagger;

use swagger_ui;
use actix_swagger::lib::swagger;
use crate::middlewares::{logger::init_logger, logging_middleware, auth_middleware, heartbeat_middleware };

use serde::Serialize;
use crate::db::db::{initialize_pool, get_pool };

#[derive(Serialize)]
struct MyData {
    code: i32,
    message: &'static str,
}

async fn index(path: web::Path<(i32,)>) -> impl Responder {
    let value = path.0;

    let response_data = MyData { code:200, message: "Hello, world!" };
    HttpResponse::Ok().json(response_data)
}

// 自定义错误处理程序函数
async fn handle_json_payload_error(err: PayloadError, _req: &ServiceRequest) -> actix_web::error::Error {
    // 检查 JSON 负载错误类型并返回自定义响应
    match err {
        PayloadError::Overflow => {
            HttpResponse::PayloadTooLarge()
                .body("JSON payload is too large!!! more than 2M")
                .into()
        },
        _ => {
            HttpResponse::BadRequest()
                .body("Bad request")
                .into()
        }
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    initialize_pool().await;

    let http_server = HttpServer::new(|| {
        let spec = swagger_ui::swagger_spec_file!("actix_swagger/openapi.json");
        let config = swagger_ui::Config::default();

        App::new().service(scope("/api/v1/swagger")
                .configure(swagger(spec, config)))
            .wrap(middleware::Logger::default())
//           //.wrap(auth_middleware::Auth)
            .wrap(logging_middleware::Logging)
           // .wrap(heartbeat_middleware::Heartbeat)
            .wrap(Compress::default())
            .app_data(web::Data::new(get_pool()))
            //全局JSON负载的最大大小为2MB,并配置自定义错误处理函数
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 2).error_handler(handle_json_payload_error))
            .configure(user_routes).route("/{value}", web::get().to(index))
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

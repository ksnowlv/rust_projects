use actix_web::{web, HttpResponse, Responder };
use crate::models::user::User;
use crate::models::user::UserRegisterInfo;

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/user")
            .route("/get_user", web::get().to(get_user))
            .route("/login", web::get().to(login))
            .route("/regist", web::get().to(regist))
            .route("/get_sms_code", web::get().to(get_sms_code))
            .route("/update", web::get().to(update))
            .route("/delete", web::get().to(delete))
            );
}

pub  async fn  get_user() -> impl Responder{
    let user = User{id:1, user_id: "abc".to_string(), phone: "15210".to_string(), age: 10, name:"ksnowlv".to_string(), token:"aaaa".to_string(), sms_code:"123456".to_string()};
    HttpResponse::Ok().json(user)
}

pub async fn regist(info: web::Json<UserRegisterInfo>) -> impl Responder {
    println!("UserRegisterInfo:{:?}", info);

    HttpResponse::Ok().body("regist successfully")
}

pub async fn login() -> impl Responder {
    HttpResponse::Ok().body("login successfully")
}

pub async fn get_sms_code() -> impl Responder {
    HttpResponse::Ok().body("get_sms_code successfully")
}

pub async fn update() -> impl Responder {
    HttpResponse::Ok().body("user updated successfully")
}

pub async fn delete() -> impl Responder {
    HttpResponse::Ok().body("delete user successfully")
}

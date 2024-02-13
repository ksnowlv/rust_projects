use serde::Deserialize;
use std::collections::HashMap;
use chrono::{Duration, Utc};
use actix_web::{web, HttpResponse, Responder, post};
use actix_web::web::Query;
use actix_web::web::to;
use crate::db::{user_db::*, db::get_pool};
use crate::models::user_model::{User, UserRegisterInfo, TokenQuery};
use crate::models::{api_response, userid_generator, token_generator};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::scope("/api/user")
			.route("/login", web::post().to(login))
			.route("/regist", web::post().to(regist))
			.route("/get_sms_code", web::get().to(get_sms_code))
			.route("/update", web::post().to(update))
			.route("/delete", web::post().to(delete))
	);
}

pub async fn regist(info: web::Json<UserRegisterInfo>) -> impl Responder {
	match get_user_by_phone(&get_pool(), &info.phone).await {
		Ok(Some(user)) => api_response::success_handler(user, None).await,
		Ok(None) => {
			let user_id = userid_generator::generate_user_id();
			let expiration_time = Utc::now().checked_add_signed(Duration::hours(1)).unwrap().timestamp() as usize;
			let token = token_generator::generate_jwt_token(&user_id, expiration_time).expect("Failed to generate JWT token");
			println!("Generated JWT token: {}", token);
			match create_user(&get_pool(), &user_id, &info.name, info.age, &info.phone, "123456", &token).await {
				Ok(res_user) => api_response::success_handler(res_user, Some("注册成功".to_string())).await,
				Err(err) => {
					println!("Error occurred: {:?}", err);
					api_response::error_handler::<String>(err.to_string().as_str()).await
				}
			}
		},
		Err(e) => api_response::error_handler::<String>(e.to_string().as_str()).await,
	}
}

pub async fn login(info: web::Json<UserRegisterInfo>) -> impl Responder {
	match get_user_by_phone(&get_pool(), &info.phone).await {
		Ok(Some(user)) => {
			if user.verification_code == info.verification_code {
				let expiration_time = Utc::now().checked_add_signed(Duration::hours(1)).unwrap().timestamp() as usize;
				let token = token_generator::generate_jwt_token(&user.userid, expiration_time).expect("Failed to generate JWT token");
				println!("Generated JWT token: {}", token);
				match update_user_token(&get_pool(), &user.userid, &token).await {
					Ok(()) => {
						let json_response = serde_json::json!({
                            "userid": user.userid, "token": token,
                        });
						api_response::success_handler(json_response, Some("登陆成功!".to_string())).await
					},
					Err(e) => api_response::notfound_handler::<String>(e.to_string().as_str()).await,
				}
			} else {
				api_response::notfound_handler::<String>("手机验证码错误，请检查后再试").await
			}
		},
		Ok(None) => api_response::notfound_handler::<String>("该用户未注册账号，请先去注册账号，谢谢").await,
		Err(e) => api_response::error_handler::<String>(e.to_string().as_str()).await,
	}
}

pub async fn get_sms_code() -> impl Responder {
	HttpResponse::Ok().body("get_sms_code successfully")
}

pub async fn update(query: Query<TokenQuery>, info: web::Json<UserRegisterInfo>) -> impl Responder {
	match get_user_by_token(&get_pool(), &query.token).await {
		Ok(Some(mut user)) => {
			match update_user(&get_pool(), &info, &user.userid).await {
				Ok(()) => api_response::ok_handler(Some(user), Some("更新成功".to_string())).await,
				Err(sqlx::Error::RowNotFound) => api_response::notfound_handler::<String>("未找到该用户，请检查更新的账号是否正确？").await,
				Err(_) => api_response::error_handler::<String>("未知错误".to_string().as_str()).await,
			}
		},
		Ok(None) => api_response::notfound_handler::<String>("未找到该用户，请检查更新的账号是否正确？").await,
		Err(e) => api_response::error_handler::<String>(e.to_string().as_str()).await,
	}
}

pub async fn delete(query: Query<TokenQuery>) -> impl Responder {
	match get_user_by_token(&get_pool(), &query.token).await {
		Ok(Some(mut user)) => {
			match delete_user(&get_pool(), &user.userid).await {
				Ok(()) => api_response::ok_handler(Some(user), Some("用户注销成功".to_string())).await,
				Err(sqlx::Error::RowNotFound) => api_response::notfound_handler::<String>("未找到该用户，注销失败!").await,
				Err(_) => api_response::error_handler::<String>("未知错误".to_string().as_str()).await,
			}
		},
		Ok(None) => api_response::notfound_handler::<String>("未找到该用户").await,
		Err(e) => api_response::error_handler::<String>(e.to_string().as_str()).await,
	}
}


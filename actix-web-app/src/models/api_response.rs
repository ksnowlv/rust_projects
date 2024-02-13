use actix_web::{web, App, HttpServer, HttpResponse, Result, error::Error as ActixError};
use serde::{Serialize, Deserialize};
use serde_json::Value;

// 定义通用的返回数据结构体
#[derive(Serialize, Deserialize)]
struct ApiResponse<T> {
	code: i32,
	message: String,
	data: Option<T>,
}


pub async fn ok_handler<T>(data: Option<T>, message: Option<String>) -> Result<HttpResponse, ActixError>
	where T: Serialize,
{
	// 构造成功的返回数据
	let response: ApiResponse<T> = ApiResponse {
		code: 200,
		message: message.unwrap_or("请求成功".to_string()), // 使用 unwrap_or 设置默认消息
		data: data,
	};

	Ok(HttpResponse::Ok().json(response))
}

// 定义通用的成功状态处理函数
pub async fn success_handler<T>(data: T, message: Option<String>) -> Result<HttpResponse, ActixError>
	where T: Serialize,
{
	// 构造成功的返回数据
	let response: ApiResponse<T> = ApiResponse {
		code: 200,
		message: message.unwrap_or("请求成功".to_string()), // 使用 unwrap_or 设置默认消息
		data: Some(data),
	};

	Ok(HttpResponse::Ok().json(response))
}

// 定义通用的失败状态处理函数
pub async fn notfound_handler<T>(error_message: &str)  -> Result<HttpResponse, ActixError>
	where T: Serialize, {
	let response: ApiResponse<T> = ApiResponse {
		code: 404,
		message: error_message.to_string(),
		data: None,
	};

	Ok(HttpResponse::NotFound().json(response))
}

//pub async fn error_handler<T>(code: i32, error_message: &str)  -> Result<HttpResponse, ActixError>
//	where T: Serialize, {
//	let response: ApiResponse<T> = ApiResponse {
//		code: code,
//		message: error_message.to_string(),
//		data: None,
//	};
//
//	Ok(HttpResponse::new(code).body(response))
//}

//// 定义通用的失败状态处理函数
pub async fn error_handler<T>(error_message: &str)  -> Result<HttpResponse, ActixError>
	where T: Serialize, {
	let response: ApiResponse<T> = ApiResponse {
		code: 400,
		message: error_message.to_string(),
		data: None,
	};

	Ok(HttpResponse::BadRequest().json(response))
}

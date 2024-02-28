use std::{env, io};
use std::fs::File;
use std::io::Write;
use actix_web::{web, HttpResponse, post, Error};
use actix_multipart::Multipart;
use futures_util::{StreamExt, TryStreamExt};
use mongodb::Client;
use crate::models::{
	api_response,
	file_model::{ GridFSFileData, GridFSFileRenameRequest},
};

use crate::xgridfs;

pub fn file_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/api/file")
		.route("/upload_file", web::post().to(upload_file))
		.route("/upload_file_to_gridfs", web::post().to(upload_file_to_gridfs))
		.route("/download_file_from_gridfs/{file_id}", web::post().to(download_file_from_gridfs))
		.route("/rename_file_from_gridfs/{file_id}/{file_name}", web::post().to(rename_file_from_gridfs))
		.route("rename_file_from_gridfs_with_json", web::post().to(rename_file_from_gridfs_with_json))
		.route("/delete_file_from_gridfs/{file_id}", web::post().to(delete_file_from_gridfs))
		.route("/find_file_from_gridfs/{file_name}", web::post().to(find_file_from_gridfs))
	);
}

//#[post("/upload_file")]
async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {

	while let Ok(Some(mut field)) = payload.try_next().await {
		let content_disposition = field.content_disposition();
		let file_name = content_disposition.get_filename().unwrap();

		// 获取临时目录路径
		let mut file_path = env::temp_dir();
		file_path.push(file_name);
		println!("---full file_path:{}, file_name:{}", file_path.display(), file_name);

		let mut file = File::create(file_path)?;
		while let Some(chunk) = field.next().await {
			let data = chunk?;
			file.write_all(&data)?;
		}
	}

	api_response::ok_handler::<String>(None, Some("文件上传成功!!!".to_string())).await
}

// 定义上传文件到GridFS的处理函数
pub async fn upload_file_to_gridfs(payload: Multipart, client: web::Data<Client>) -> Result<HttpResponse, Error> {
	// 调用xgridfs模块中的上传函数
	match xgridfs::upload_file_to_gridfs(payload, &client).await {
		Ok(file_data) => {
			api_response::ok_handler::<GridFSFileData>(
				Some(file_data),
				Some("文件上传处理完成".to_string()),
			).await
		}
		Err(err) => {
			// 返回文件上传失败的响应
			api_response::error_handler::<String>(err.to_string().as_str()).await
		}
	}
}

// 定义从GridFS下载文件的处理函数
pub async fn download_file_from_gridfs(file_id: web::Path<String>, client: web::Data<Client>) -> Result<HttpResponse, Error> {
	match xgridfs::download_file_from_gridfs(&file_id.to_string(), &client).await {
		Ok(file_data) => {
			// 返回文件下载成功的响应
			api_response::ok_handler(Some(file_data), Some("下载成功".to_string())).await
		}
		Err(err) => {
			// 返回文件下载失败的响应
			api_response::error_handler::<String>(err.to_string().as_str()).await
		}
	}
}

// 定义从GridFS重命名文件的处理函数
pub async fn rename_file_from_gridfs(info: web::Path<(String, String)>, client: web::Data<Client>) -> Result<HttpResponse, Error> {
	match xgridfs::rename_file_from_gridfs(info.0.as_str(), info.1.as_str(), &client).await {
		Ok(info) => {
			// 返回文件下载成功的响应
			api_response::ok_handler(Some(info), Some("文件重命名成功".to_string())).await
		}
		Err(err) => {
			api_response::error_handler::<String>(err.to_string().as_str()).await
		}
	}
}

// 定义从GridFS重命名文件的处理函数
pub async fn rename_file_from_gridfs_with_json(req: web::Json<GridFSFileRenameRequest>, client: web::Data<Client>) -> Result<HttpResponse, Error> {
	match xgridfs::rename_file_from_gridfs(&req.file_id, &req.file_name, &client).await {
		Ok(info) => {
			// 返回文件下载成功的响应
			api_response::ok_handler(Some(info), Some("文件重命名成功".to_string())).await
		}
		Err(err) => {
			api_response::error_handler::<String>(err.to_string().as_str()).await
		}
	}
}

// 定义从GridFS删除文件的处理函数
pub async fn delete_file_from_gridfs(info: web::Path<(String,)>, client: web::Data<Client>) -> Result<HttpResponse, Error> {
	match xgridfs::delete_file_from_gridfs(info.0.as_str(), &client).await {
		Ok(info) => {
			// 返回文件下载成功的响应
			api_response::ok_handler(Some(info), Some("文件删除成功".to_string())).await
		}
		Err(err) => {
			api_response::error_handler::<String>(err.to_string().as_str()).await
		}
	}
}

pub async fn find_file_from_gridfs(info: web::Path<(String,)>, client: web::Data<Client>) -> Result<HttpResponse, Error> {
	match xgridfs::find_file_from_gridfs(info.0.as_str(), &client).await {
		Ok(info) => {
			// 返回文件下载成功的响应
			api_response::ok_handler(Some(info), Some("文件查找成功".to_string())).await
		}
		Err(err) => {
			api_response::error_handler::<String>(err.to_string().as_str()).await
		}
	}
}


//	let db = client.database("mydb");
//	let mut bucket = GridFSBucket::new(db.clone(), Some(GridFSBucketOptions::default()));
//
//	while let Ok(Some(mut field)) = payload.try_next().await {
//		let file_name = {
//			let content_disposition = field.content_disposition();
//			content_disposition.get_filename().unwrap()
//		};
//
//		let upload_file_name: String = String::from(file_name);
//		let content_type = field.content_type().unwrap();// 获取文件的内容类型
//		let mut file_data = Vec::new();
//
//		while let Some(chunk) = field.next().await {
//			let data = chunk?;
//			file_data.extend_from_slice(&data);
//		}
//
//		let upload_file_data: &[u8] = &file_data;
//
//		let upload_res = bucket.upload_from_stream(&upload_file_name, upload_file_data, None).await;
//
//		match upload_res {
//			Ok(objectid) => {
//				println!("objectid = {}", objectid.to_hex());
//			}
//			Err(err) => {
//				println!("upload error = {:?}", err);
//			}
//		}
//	}

//	let serialized_entries: Vec<GridFSFileEntrySerialized> = entries
//		.into_iter()
//		.map(|entry| GridFSFileEntrySerialized {
//			filename: entry.filename,
//			filetype: entry.filetype,
//			fileid: entry.fileid,
//		})
//		.collect();
//
//	// 将可序列化类型转换为 JSON 字符串
//	let json_string = serde_json::to_string(&serialized_entries).expect("Failed to convert to JSON");
//

//	api_response::ok_handler::<String>(None, Some("文件上传成功!!!".to_string())).await
//}




//pub async fn download_file_from_gridfs(file_id: web::Path<String>, client: web::Data<Client>) -> Result<HttpResponse, Error> {
//	let bucket = create_gridfs_bucket(&client).await;
//
//	let file_id = ObjectId::with_string(&file_id)?;
//	let gridfs_file = bucket.open_download_stream(file_id).await?;
//	let mut file_content = Vec::new();
//	gridfs_file.read_to_end(&mut file_content).await?;
//
//	Ok(HttpResponse::Ok()
//		.content_type("application/octet-stream")
//		.body(file_content))
//}








use std::env;
use std::fs::File;
use std::io::Write;
use actix_web::{web, HttpResponse, post};
use actix_multipart::Multipart;
use futures_util::{StreamExt, TryStreamExt};
use actix_web::Error;
use crate::models::{api_response};

pub fn file_routes(cfg: &mut web::ServiceConfig) {
	cfg.service(web::scope("/api/file")
					.route("/upload_file", web::post().to(upload_file))
					.route("/index_file", web::post().to(index_file))
//           .service(upload_file)
	);
}
//
//async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
//
//       println!("upload_file");
//
//       while let Some (mut field) = payload.try_next().await?{
//              let content_disposition = field.content_disposition();
//              let file_name = content_disposition.get_filename().unwrap();
//
//              let file_path = format!("/tmp/{}",file_name);
//              let mut file = File::create(file_path)?;
//
//              while let Some(chunk) = field.next().await {
//                     let data_chunk = chunk.map_err(|e| {
//                            log::error!("Error while receiving file data: {:?}", e);
//                            actix_web::error::ErrorInternalServerError("Error while receiving file data")
//                     })?;
//
//                     match file.write_all(&data_chunk) {
//                            Err(e) => return Err(Error::from(e)),
//                            _ => (),
//                     }
//              }
//       }
//
//       api_response::notfound_handler::<String>("手机验证码错误，请检查后再试").await
//
//}

async fn index_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
	// iterate over multipart stream
	while let Some(item) = payload.next().await {
		let mut field = item?;

		// Field in turn is stream of *Bytes* object
		while let Some(chunk) = field.next().await {
			println!("-- CHUNK: \n{:?}", std::str::from_utf8(&chunk?));
		}
	}

	// Ok(HttpResponse::Ok().into())
	api_response::ok_handler::<String>(None, Some("自定义消息".to_string())).await
//	api_response::notfound_handler::<String>("手机验证码错误，请检查后再试").await
}


//#[post("/upload_file")]
async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
	println!("\n---upload_file---\n");

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





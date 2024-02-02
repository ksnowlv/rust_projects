
use actix_web::{web, HttpResponse };
use actix_multipart::Multipart;


pub fn file_routes(cfg: &mut web::ServiceConfig) {
       cfg.service(web::scope("/api/file")
           .route("/upload_file", web::post().to(upload_file))
       );
}

async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {

       println!("upload_file");
//       while let Ok(Some(mut field)) = payload.try_next().await {
//              // 获取上传文件的文件名
//              let content_type = field.content_disposition();
//              let filename = content_type.get_filename().ok_or_else(|| error::ErrorBadRequest("missing file"))?;
//       }

       Ok(HttpResponse::Ok().body("File uploaded"))
}

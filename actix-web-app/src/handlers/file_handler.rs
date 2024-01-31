
use actix_web::{web, HttpResponse, Responder };


use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use uuid::Uuid;

async fn upload_file(mut payload: Multipart) -> Result<HttpResponse> {
       while let Ok(Some(mut field)) = payload.try_next().await {
              let content_type = field.content_disposition().unwrap();
              let filename = Uuid::new_v4().to_string();
              let filepath = format!("/path/to/save/{}", filename);

              let mut f = web::block(|| std::fs::File::create(filepath))
                  .await
                  .unwrap();

              while let Some(chunk) = field.next().await {
                     let data = chunk.unwrap();
                     f = web::block(move || f.write_all(&data).map(|_| f)).await?;
              }
       }

       Ok(HttpResponse::Ok().body("File uploaded"))
}
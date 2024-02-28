use actix_multipart::Multipart;
use actix_web::Error;
use chrono::{TimeZone, Utc};
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
	bson::doc,
	Client,
	bson::oid::ObjectId,
};
use mongodb_gridfs::{
	GridFSBucket,
	options::{GridFSBucketOptions, GridFSFindOptions},
};
use sqlx::testing::TestTermination;
use crate::models::{
	file_model::{GridFSFileDownloadInfo, GridFSFileData, GridFSFileRenameInfo, GridFSFileInfo, GridFSDocumentInfo}
};

use crate::md5_hash::md5_data;

pub async fn upload_file_to_gridfs(mut payload: Multipart, client: &Client) -> Result<GridFSFileData, actix_web::Error> {
	let db = client.database("mydb");
	let mut bucket = GridFSBucket::new(db.clone(), Some(GridFSBucketOptions::default()));

	let mut gridfs_file_data = GridFSFileData::new();

	while let Ok(Some(mut field)) = payload.try_next().await {
		let file_name = {
			let content_disposition = field.content_disposition();
			content_disposition.get_filename().unwrap().clone()
		};

		let mut file_info = GridFSFileInfo::new_default();

		file_info.set_file_name(file_name.to_string());

		let content_type = field.content_type().unwrap().to_string();
		file_info.set_file_type(content_type);

		let mut file_data = Vec::new();

		while let Some(chunk) = field.next().await {
			let data = chunk?;
			file_data.extend_from_slice(&data);
		}

		let res = find_file_in_gridfs(&mut  bucket, &file_info.file_name, &file_data).await;

		match res {
			Ok(info) => {
				// 处理文件找到的情况
				println!(" file info:{:?} ", info);
				file_info.uploaded = true;
				file_info.set_file_id(info.objectid);
			}
			Err(err) => {
				let upload_file_data: &[u8] = &file_data;
				let upload_res = bucket.upload_from_stream(&file_info.file_name, upload_file_data, None).await;

				match upload_res {
					Ok(objectid) => {
						println!("objectid = {}", objectid.to_hex());
						file_info.set_file_id(objectid.to_hex());
						file_info.set_success(true);
					}
					Err(err) => {
						println!("upload error = {:?}", err);
					}
				}
			}
		}

		gridfs_file_data.push_file_entry(file_info);
	}

	Ok(gridfs_file_data)
}

pub async  fn  find_file_in_gridfs(bucket: &mut GridFSBucket, file_name: &str, file_data: &Vec<u8>) -> Result<GridFSDocumentInfo, Box<dyn std::error::Error>> {

	let mut cursor = bucket
		.find(doc! {"filename":file_name}, GridFSFindOptions::default())
		.await?;

	let mut is_found = false;
	let mut info = GridFSDocumentInfo{objectid: "".to_string(),
									  filename: "".to_string() ,
									  chunk_size: 0,
									  length: 0,
									  upload_date:None};
	// 遍历 Cursor<Document> 并获取 Document 信息
	while let Some(doc) = cursor.next().await {
		match doc {
			Ok(document) => {
				// 处理查询到的文档
				println!("Found document: {:?}", document);

				if let Some(_id) = document.get("_id").and_then(|v| v.as_object_id()) {
					println!("File objectid: {}", _id.to_hex());
					info.objectid = _id.to_hex();
				}

				if let Some(file_name) = document.get("filename").and_then(|v| v.as_str()) {
					println!("File Name: {}", file_name);
					info.filename = file_name.to_string();
				}

				if let Some(chunk_size) = document.get("chunkSize").and_then(|v| v.as_i32()) {
					println!("File chunkSize: {}", chunk_size);
					info.chunk_size = chunk_size;
				}

				if let Some(length) = document.get("length").and_then(|v| v.as_i64()) {
					println!("File length: {}", length);
					info.length = length;
				}

				if let Some(upload_date) = document.get("uploadDate").and_then(|v| v.as_datetime()) {
					println!("Upload Date: {}", upload_date);
					info.upload_date = Some(*upload_date);
				}
				if let Some(length) = document.get("length").and_then(|v| v.as_i64()) {
					println!("File Size: {} bytes", length);
				}

				if let Some(file_md5) = document.get("md5").and_then(|v| v.as_str()) {
					println!("File md5:{}", file_md5);
					let md5_string = md5_data(&file_data);
					let result = md5_string.ok_or_else(||
						actix_web::error::ErrorBadRequest("MD5 string is missing")
					)?;

					if result == file_md5 {
						//actix_web::error::ErrorNotFound("MD5 not match")
						is_found = true;
					}
				}
			}
			Err(err) => {
				println!("Error while iterating cursor: {}", err);
				return Err(Box::new(err));
			}
		}
	}

	if is_found {
		Ok(info)
	} else {
		Err(Box::new(actix_web::error::ErrorNotFound("File not found")))
	}
}

pub async fn download_file_from_gridfs(file_id: &str, client: &Client) -> Result<GridFSFileDownloadInfo, Box<dyn std::error::Error>> {
	let db = client.database("mydb");
	let mut bucket = GridFSBucket::new(db.clone(), Some(Default::default()));
	let file_id_obj = ObjectId::parse_str(file_id)?;


	let (mut file, filename) = bucket.open_download_stream_with_filename(file_id_obj).await?;

	let buffer = file.next().await.unwrap();

	let info = GridFSFileDownloadInfo {
		filename: filename,
		content_type: "to do".to_string(),
		data: buffer,
	};

	Ok(info)
}

pub async fn rename_file_from_gridfs(file_id: &str, new_filename: &str, client: &Client) -> Result<GridFSFileRenameInfo, Box<dyn std::error::Error>> {
	let db = client.database("mydb");
	let mut bucket = GridFSBucket::new(db.clone(), Some(Default::default()));
	let file_id_obj = ObjectId::parse_str(file_id)?;
	let mut update_result = bucket.rename(file_id_obj, new_filename).await?;

	println!("matched_count:{}, modified_count: {}, upserted_id:{:?}",
			 update_result.matched_count,
			 update_result.modified_count, update_result.upserted_id);
	let info = GridFSFileRenameInfo {
		modified_count:update_result.modified_count
	};

	Ok(info)
}

pub async fn delete_file_from_gridfs(file_id: &str, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
	let db = client.database("mydb");
	let mut bucket = GridFSBucket::new(db.clone(), Some(Default::default()));
	let file_id_obj = ObjectId::parse_str(file_id)?;
	let result = bucket.delete(file_id_obj).await?;
	println!("File deleted successfully");
	Ok(())
}

pub async fn find_file_from_gridfs(file_name: &str, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
	let db = client.database("mydb");
	let mut bucket = GridFSBucket::new(db.clone(), Some(Default::default()));

	let mut cursor = bucket
		.find(doc! {"filename":file_name}, GridFSFindOptions::default())
		.await?;

	// 遍历 Cursor<Document> 并获取 Document 信息
	while let Some(doc) = cursor.next().await {
		match doc {
			Ok(document) => {
				// 处理查询到的文档
				println!("Found document: {:?}", document);
				// 你可以在这里对 document 进行进一步的处理
				if let Some(file_name) = document.get("filename").and_then(|v| v.as_str()) {
					println!("File Name: {}", file_name);
				}
				if let Some(upload_date) = document.get("uploadDate").and_then(|v| v.as_i64()) {
					let upload_date = Utc.timestamp(upload_date, 0);
					println!("Upload Date: {}", upload_date.to_rfc3339());
				}
				if let Some(file_size) = document.get("length").and_then(|v| v.as_i64()) {
					println!("File Size: {} bytes", file_size);
				}

				if let Some(file_md5) = document.get("md5").and_then(|v |v.as_str()) {
					println!("File md5:{}", file_md5);
				}
			}
			Err(err) => {
				println!("Error while iterating cursor: {}", err);
				break;
			}
		}
	}

	Ok(())
}


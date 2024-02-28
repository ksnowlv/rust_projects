use mongodb::bson::DateTime;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct GridFSFileData {
	pub file_list: Vec<GridFSFileInfo>,
}

impl GridFSFileData {

	pub fn new() -> Self {
		GridFSFileData {
			file_list:Vec::new()
		}
	}

	pub fn push_file_entry(& mut self, file_info: GridFSFileInfo) {
		self.file_list.push(file_info);
	}
}

#[derive(Debug, Serialize)]
pub struct GridFSFileInfo {
	pub file_name: String,
	pub file_type: String,
	pub file_id: String,
	pub success: bool,
	pub uploaded: bool,
}

impl GridFSFileInfo {
	pub fn new_default() -> Self {
		GridFSFileInfo {
			file_name: "".to_string(),
			file_type: "".to_string(),
			file_id: "".to_string(),
			success: false,
			uploaded: false,
		}
	}

	pub fn set_file_name(&mut self, file_name: String) {
		self.file_name = file_name;
	}

	pub fn set_file_type(&mut self, file_type: String) {
		self.file_type = file_type;
	}

	pub fn set_file_id(&mut self, file_id: String) {
		self.file_id = file_id;
	}

	pub fn set_success(&mut self, success: bool) {
		self.success = success;
	}
}

#[derive(Debug, Serialize)]
pub struct GridFSFileDownloadInfo {
	pub filename: String,
	pub content_type: String,
	pub data: Vec<u8>,
}

#[derive(Debug, Serialize)]
pub struct GridFSFileRenameInfo {
	pub modified_count: u64,
}


#[derive(Debug, Serialize)]
pub struct GridFSDocumentInfo {
	pub objectid: String,
	pub filename: String,
	pub chunk_size: i32,
	pub length: i64,
	pub upload_date: Option<DateTime>,
}


#[derive(Deserialize)]
pub struct GridFSFileRenameRequest {
	pub file_id: String,
	pub file_name: String,
}



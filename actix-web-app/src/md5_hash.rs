use openssl::hash::{hash, MessageDigest};


pub fn md5_data(data: &[u8]) -> Option<String> {
	let result = hash(MessageDigest::md5(), data).ok()?;
	let hex_string = result.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();
	Some(hex_string)
}

use chrono::Utc;
use jsonwebtoken::Algorithm;
use serde_json::json;
use jsonwebtoken::{decode, encode, Header, Validation, EncodingKey, DecodingKey};
use rand::Rng;
use rand::thread_rng;
use std::time::Duration;
use jsonwebtoken::Algorithm::HS512;
use serde_derive::{Deserialize, Serialize};

pub const TOKEN_SECRET: &str = "123456helloworld*_++";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
	userid: String,
	exp: usize,
}

// 生成一个随机的 jwt_secret 密钥
pub fn generate_jwt_secret() -> String {
	let rng = &mut thread_rng();
	let bytes = rng.gen::<[u8; 32]>();
	base64::encode(&bytes)
}

// 生成一个 JWT token
pub fn generate_jwt_token(user_id: &str, expiration_time: usize) -> Result<String, jsonwebtoken::errors::Error> {
	let claims = TokenClaims {
		userid:user_id.to_string(),
		exp: expiration_time,
	};

	let header = Header::new(Algorithm::HS256);
	encode(&header, &claims, &EncodingKey::from_secret(TOKEN_SECRET.as_ref()))
}

// 验证 JWT token
pub fn validate_jwt_token(token: &str) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
	let decoding_key = DecodingKey::from_secret(TOKEN_SECRET.as_ref());
	let mut  validation= Validation::default();// Algorithm::HS256 指定算法，根据实际使用的算法进行调整
	validation.validate_exp = true;// 启用过期时间验证

	let result = decode::<TokenClaims>(token, &decoding_key, &validation)?;
	Ok(result.claims)
}


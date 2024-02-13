use sqlx::mysql::MySqlRow;
use sqlx::{Pool, Row, FromRow, MySql};
use serde::{Serialize, Deserialize};
use crate::models::userid_generator::{generate_user_id};
use crate::models::user_model::{User, UserRegisterInfo };
use std::fmt::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
	pub name: String,
	pub age: u32,
	pub phone: String,
	pub token: String,
}

#[derive(FromRow, Serialize)]
pub struct UserInfo {
	pub userid: String,
	pub name: String,
	pub age: u32,
	pub phone: String,
}

pub async fn create_user(pool: &Pool<sqlx::MySql>,  userid: &str, name: &str, age: i32, phone: &str, sms_code: &str, token: &str) -> Result<User, sqlx::Error> {
	let mut user = User {
		id: 0,
		userid:userid.to_string(),
		name:name.to_string(),
		age:age,
		phone:phone.to_string(),
		verification_code:sms_code.to_string(),
		token: token.to_string()
	};

	let mut connection = pool.acquire().await?;
	let result = sqlx::query(
		r#"INSERT INTO users (userid, name, age, phone, verification_code, token)
        VALUES (?, ?, ?, ?, ?, ?)"#, )
		.bind(userid)
		.bind(name)
		.bind(age)
		.bind(phone)
		.bind(sms_code)
		.bind(token)
		.execute(&mut connection)
		.await?;

	if result.rows_affected() > 0 {
		user.id = result.last_insert_id();
		Ok(user)
	} else {
		Err(sqlx::Error::RowNotFound)
	}
}

pub async fn get_user_by_token(pool: &Pool<sqlx::MySql>, token: &str) -> Result<Option<User>, sqlx::Error> {
	let mut connection = pool.acquire().await?;
	let user = sqlx::query_as::<_, User>(r#"SELECT id, userid, phone, age, name, token, verification_code  FROM users WHERE token = ?"#, )
		.bind(token)
		.fetch_optional(&mut connection)
		.await?;

	Ok(user)
}

pub async fn get_user_by_phone(pool: &Pool<sqlx::MySql>, phone: &str) -> Result<Option<User>, sqlx::Error> {
	let mut connection = pool.acquire().await?;
	let user = sqlx::query_as::<_, User>(r#"SELECT id, userid, phone, age, name, token, verification_code  FROM users  WHERE phone = ?"#, )
		.bind(phone)
		.fetch_optional(&mut connection)
		.await?;

	Ok(user)
}

pub async fn update_user(pool: &Pool<sqlx::MySql>, user: &UserRegisterInfo, userid: &str) -> Result<(), sqlx::Error> {
	let mut connection = pool.acquire().await?;
	let result = sqlx::query(r#"UPDATE users SET name = ?, age = ?, phone = ? WHERE userid = ?"#, )
		.bind(&user.name)
		.bind(user.age)
		.bind(&user.phone)
		.bind(userid)
		.execute(&mut  connection)
		.await?;

	if result.rows_affected() > 0 {
		Ok(())
	} else {
		Err(sqlx::Error::RowNotFound)
	}
}

pub async fn update_user_token(pool: &Pool<sqlx::MySql>, user_id: &str, user_token: &str) -> Result<(), sqlx::Error> {
	let mut connection = pool.acquire().await?;
	let result = sqlx::query(r#"UPDATE users SET token = ? WHERE userid = ?"#, )
		.bind(user_token)
		.bind(user_id)
		.execute(&mut  connection)
		.await?;

	if result.rows_affected() > 0 {
		Ok(())
	} else {
		Err(sqlx::Error::RowNotFound)
	}
}

pub async fn delete_user(pool: &Pool<sqlx::MySql>, user_id: &str) -> Result<(), sqlx::Error> {
	let mut connection = pool.acquire().await?;
	let result = sqlx::query(r#"DELETE FROM users WHERE userid = ?"#, )
		.bind(user_id)
		.execute(&mut connection)
		.await?;

	if result.rows_affected() > 0 {
		Ok(())
	} else {
		Err(sqlx::Error::RowNotFound)
	}
}


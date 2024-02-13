use once_cell::sync::OnceCell;
use sqlx::mysql::MySqlPool;
use sqlx::Pool;
use std::sync::Arc;
use sqlx::error::Error as SqlxError;
use sqlx::error::DatabaseError;


pub const MYSQL_DATABASE_URL: &str = "mysql://root:111111@127.0.0.1:3306/mydatabase?charset=utf8mb4";

static DB_POOL: OnceCell<Arc<MySqlPool>> = OnceCell::new();

pub async fn initialize_pool() {

	let database_url = MYSQL_DATABASE_URL;

	let pool = MySqlPool::connect(database_url).await
		.expect("Failed to create pool");
	let pool = Arc::new(pool);
	DB_POOL.set(pool).expect("Failed to set DB_POOL");
	println!("---initialize_pool---");
}

pub fn get_pool() -> Arc<MySqlPool> {
	DB_POOL.get().expect("DB_POOL not initialized").clone()
}


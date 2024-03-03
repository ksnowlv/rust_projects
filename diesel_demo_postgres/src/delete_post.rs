use diesel::prelude::*;
use crate::postgres_db::db_delete_post;

pub fn delete_post(connection: &mut PgConnection) {

    println!("---delete_post---");
    let target = "ff";
    let pattern = format!("%{}%", target);

    db_delete_post(connection, pattern.as_str());
}
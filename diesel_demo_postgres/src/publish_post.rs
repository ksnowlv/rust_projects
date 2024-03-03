use diesel::prelude::*;
use crate::postgres_db::db_update_post;

pub fn publish_post(connection : &mut PgConnection, post_id: i32) {
    db_update_post(connection, post_id);
}
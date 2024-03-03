
use diesel::prelude::*;
use crate::postgres_db::{db_get_post, db_show_all_posts, db_show_posts};

pub fn get_post(connection: &mut PgConnection) {

    println!("---get_post （find id = 2的数据项）---");
    db_get_post(connection, 2);
}

pub fn show_posts(connection: &mut PgConnection) {
    println!("---show_posts---");
    db_show_posts(connection, 10);
}

pub fn show_all_posts(connection: &mut PgConnection) {
    println!("---show_all_posts---");
   db_show_all_posts(connection);
}
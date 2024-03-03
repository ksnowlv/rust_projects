
mod models;
mod postgres_db;
mod publish_post;
mod show_posts;
mod create_post;
mod schema;
mod delete_post;

use crate::show_posts::{show_posts, show_all_posts, get_post};
use crate::create_post::create_post;
use crate::publish_post::publish_post;
use crate::delete_post::delete_post;
use crate::postgres_db::db_establish_connection;

fn main() {

    let connection = &mut db_establish_connection();

    create_post(connection);
    show_all_posts(connection);

    publish_post(connection, 34);
    publish_post(connection, 35);
    show_posts(connection);

    get_post(connection);
    show_all_posts(connection);

    delete_post(connection);
}
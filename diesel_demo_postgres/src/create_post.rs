
use diesel::PgConnection;
use crate::postgres_db::{db_create_post};
use std::io::{stdin};

pub fn create_post(connection: &mut PgConnection) {

    loop {
        let mut title = String::new();
        let mut body = String::new();

        println!("What would you like your title to be?");
        stdin().read_line(&mut title).unwrap();
        let title = title.trim_end();

        println!("\nOk! Let's write {title} if you input end,input is terminal ",);
        stdin().read_line(&mut body).unwrap();

        if body.contains("end") {
            if let Some(index) = body.rfind("end") {
                body = body[..index].to_string(); // 使用切片操作来截取需要的部分
                println!("body:{}", body);
            }
            break;
        }

        let post = db_create_post(connection, title, &body);
        println!("\nSaved draft {title} with id {}", post.id);
    }
}

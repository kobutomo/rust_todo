#[macro_use]
extern crate diesel;
extern crate dotenv;

use chrono;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::NewTodo;
use std::env;

mod models;
mod schema;

use schema::todos::dsl::*;

/// DBの接続を確立
fn establish_connection() -> PgConnection {
    dotenv().ok();

    // .envファイルに定義された環境変数DATABASE_URLを取得してDBに接続する
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    // DBの接続を確立
    let conn = establish_connection();

    // customerテーブルの各情報を取得
    // SQLで「SELECT * FROM customer;」をやっているのと同じ
    let result = todos.filter(name.eq("test")).first::<models::Todo>(&conn);

    match result {
        Ok(file) => file,
        Err(_error) => {
            let new_todo = NewTodo {
                name: "test".to_string(),
                detail: Option::None,
                date_from: Option::Some(chrono::NaiveDateTime::parse_from_str(
                    "2021-09-20 5:00:00",
                    "%Y-%m-%d %H:%M:%S",
                ).expect("failed to parse")),
                date_to: Option::Some(chrono::NaiveDateTime::parse_from_str(
                    "2021-09-20 6:00:00",
                    "%Y-%m-%d %H:%M:%S",
                ).expect("failed to parse")),
            };

            let database_record = diesel::insert_into(schema::todos::table)
                .values(&new_todo)
                .get_result::<models::Todo>(&conn)
                .expect("Error saving new example");
            database_record
        }
    };

    let results = todos
        .load::<models::Todo>(&conn)
        .expect("Error loading customer");

    // 結果を表示
    for r in results {
        println!("{:?}", r);
    }
}

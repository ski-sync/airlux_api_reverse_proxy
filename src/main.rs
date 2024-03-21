#[macro_use]
extern crate diesel;

pub mod db;
pub mod port;
pub mod routes;

use actix_web::{
    web::{self},
    App, HttpServer,
};
use core::fmt;
use db::{DatabaseConnection, Pool};
use diesel::r2d2::ConnectionManager;
use std::{fmt::Display, fs};

/// Helps with changing the database engine without much edits.

// fn get_used_ports() -> Vec<u32> {
//     // open file and read used ports
//     let data = fs::File::open("used_ports.json").unwrap();
//     let used_ports: Vec<u32> = serde_json::from_reader(data).unwrap();
//     used_ports
// }

// fn merge_ports(ports: Vec<u32>, used_ports: Vec<u32>) -> Vec<u32> {
//     let mut merged_ports = used_ports;
//     for port in ports {
//         merged_ports.push(port);
//     }
//     merged_ports
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap_or("app.db".to_string());
    let database_pool = Pool::builder()
        .build(ConnectionManager::<DatabaseConnection>::new(database_url))
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database_pool.clone()))
            .service(routes::register)
            .service(routes::api_get)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}

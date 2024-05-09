#[macro_use]
extern crate diesel;

pub mod db;
pub mod port;
pub mod routes;

use actix_web::{
    web::{self},
    App, HttpServer,
};
use db::{DatabaseConnection, Pool};
use diesel::r2d2::ConnectionManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8081 ");
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap_or("app.db".to_string());
    let database_pool = Pool::builder()
        .build(ConnectionManager::<DatabaseConnection>::new(database_url))
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database_pool.clone()))
            .service(routes::register)
            .service(routes::get_ports)
            .service(routes::get_traefik_config)
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await
}

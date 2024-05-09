use actix_web::{post, App, HttpServer};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Register {
    pub ssh_key: String,
}

#[post("/register")]
async fn register(register: actix_web::web::Json<Register>) -> actix_web::HttpResponse {
    println!("{:?}", register);
    actix_web::HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8081");

    HttpServer::new(move || App::new().service(register))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}

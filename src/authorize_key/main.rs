use actix_web::{post, App, HttpServer};

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Register {
    pub ssh_key: String,
}

// this function save ssh_key in authorized_keys file
pub fn save_ssh_key(ssh_key: String) {
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut file = OpenOptions::new()
        .append(true)
        .open("/root/.ssh/authorized_keys")
        .unwrap();

    file.write_all(ssh_key.as_bytes()).unwrap();
}

#[post("/register")]
async fn register(register: actix_web::web::Json<Register>) -> actix_web::HttpResponse {
    save_ssh_key(register.ssh_key.clone());
    actix_web::HttpResponse::Ok().json("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8081");

    HttpServer::new(move || App::new().service(register))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}

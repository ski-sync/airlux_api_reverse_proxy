use actix_web::{post, App, HttpServer};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

// this struct is used to deserialize the JSON body of the POST request
#[derive(serde::Deserialize, Debug, Clone)]
pub struct Register {
    pub ssh_key: String,
}

// this function save ssh_key in authorized_keys file
async fn save_ssh_key(ssh_key: String) -> std::io::Result<()> {
    // Open authorized_keys file in append mode
    let mut file = OpenOptions::new()
        .append(true)
        .open("/root/.ssh/authorized_keys")
        .await?;

    // Ensure the ssh_key ends with a newline character
    let mut ssh_key_with_newline = ssh_key;
    if !ssh_key_with_newline.ends_with('\n') {
        ssh_key_with_newline.push('\n');
    }

    // Write the ssh_key to the file
    file.write_all(ssh_key_with_newline.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}

// this function is called when a POST request is made to /register
#[post("/register")]
async fn register(register: actix_web::web::Json<Register>) -> actix_web::HttpResponse {
    let _ = save_ssh_key(register.ssh_key.clone()).await;
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

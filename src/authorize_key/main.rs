use actix_web::{post, App, HttpServer};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

/// A struct for deserializing JSON from incoming POST requests.
/// It holds an SSH key as a string, provided by the user to register their SSH access.
#[derive(serde::Deserialize, Debug, Clone)]
pub struct Register {
    pub ssh_key: String,
}

/// Asynchronously saves an SSH key to the `authorized_keys` file.
///
/// # Arguments
/// * `ssh_key` - A string containing the user's SSH key.
///
/// # Returns
/// A Result<(), std::io::Error> indicating success or failure.
///
/// # Errors
/// This function will return an error if the file cannot be opened or written to.
async fn save_ssh_key(ssh_key: String) -> std::io::Result<()> {
    // Open the authorized_keys file in append mode to add new keys without deleting existing ones.
    let mut file = OpenOptions::new()
        .append(true)
        .open("/root/.ssh/authorized_keys")
        .await?;

    // Append a newline to the SSH key if it doesn't already end with one, ensuring format consistency.
    let mut ssh_key_with_newline = ssh_key;
    if !ssh_key_with_newline.ends_with('\n') {
        ssh_key_with_newline.push('\n');
    }

    // Write the SSH key to the file and flush the buffer to ensure it is written to disk.
    file.write_all(ssh_key_with_newline.as_bytes()).await?;
    file.flush().await?;
    Ok(())
}

/// Handles POST requests to /register, registering a new SSH key by saving it to the authorized_keys file.
///
/// # Request
/// Expects a JSON body with a single field `ssh_key`.
///
/// # Response
/// Returns a JSON response with the string "OK" if the SSH key is successfully saved.
///
/// # Example
/// ```json
/// POST /register
/// {
///     "ssh_key": "ssh-rsa AAA..."
/// }
/// ```
#[post("/register")]
async fn register(register: actix_web::web::Json<Register>) -> actix_web::HttpResponse {
    let _ = save_ssh_key(register.ssh_key.clone()).await;
    actix_web::HttpResponse::Ok().json("OK")
}

/// Entry point for the Actix Web server.
///
/// Starts an HTTP server bound to `0.0.0.0:8081` and sets up the routing to handle the `/register` endpoint.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8081");

    HttpServer::new(move || App::new().service(register))
        .bind(("0.0.0.0", 8081))?
        .run()
        .await
}

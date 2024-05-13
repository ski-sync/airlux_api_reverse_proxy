use actix_web::{get, web, Error, HttpResponse};

use crate::{
    db::{get_ports_by_mac, get_used_ports, insert_mac_address, insert_ports, Pool},
    port::Register,
};

#[get("/api/register")]
pub async fn register(
    pool: web::Data<Pool>,
    register_json: web::Json<Register>,
) -> Result<HttpResponse, Error> {
    // insert mac address and ssh key
    match insert_mac_address(
        pool.clone(),
        register_json.address_mac.clone(),
        register_json.ssh_key.clone(),
    ) {
        Ok(_) => {
            register_json.save_ssh_key();
        }
        Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            _,
        )) => {
            match get_ports_by_mac(pool.clone(), register_json.address_mac.clone()) {
                Ok(used_ports) => return Ok(HttpResponse::Ok().json(used_ports)),
                Err(e) => {
                    return Ok(HttpResponse::InternalServerError()
                        .body(format!("Error getting used ports: {}", e)))
                }
            };
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError()
                .body(format!("Error inserting mac address: {}", e)))
        }
    };

    let used_ports = match get_used_ports(pool.clone()) {
        Ok(used_ports) => used_ports,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError()
                .body(format!("Error getting used ports: {}", e)))
        }
    };

    let mut register: Register = register_json.clone();
    register.get_ports(used_ports.clone());

    match insert_ports(
        pool.clone(),
        register.address_mac.clone(),
        &register.ports.get(),
    ) {
        Ok(_) => (),
        Err(e) => {
            return Ok(
                HttpResponse::InternalServerError().body(format!("Error inserting ports: {}", e))
            )
        }
    }

    // return Vec<u32> of ports
    Ok(HttpResponse::Ok().json(
        register
            .ports
            .get()
            .iter()
            .map(|x| x.port)
            .collect::<Vec<u32>>(),
    ))
}

#[get("/api/ports")]
pub async fn get_ports(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let used_ports = match get_used_ports(pool.clone()) {
        Ok(used_ports) => used_ports,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError()
                .body(format!("Error getting used ports: {}", e)))
        }
    };
    Ok(HttpResponse::Ok().json(used_ports))
}

/// Generates the YAML configuration for Traefik based on the assigned ports.
///
/// # Arguments
/// * `pool` - A database connection pool.
async fn generate_traefik_config(pool: web::Data<Pool>) -> Result<String, Error> {
    let used_ports = match get_used_ports(pool.clone()).map_err(|e| {
        eprintln!("Error retrieving used ports: {}", e);
        HttpResponse::InternalServerError().body("Failed to retrieve port data")
    }) {
        Ok(used_ports) => used_ports,
        Err(_) => {
            return Ok(String::from(
                "http:\n  routers:\n  services:\n  middlewares:\n",
            ))
        }
    };

    let mut traefik_config = String::from("http:\n  routers:\n");
    for port in &used_ports {
        let domain = format!("box-{}.proxy.ski-sync.com", port);
        let router_block = format!(
            "    box-{port}:\n      rule: \"Host(`{domain}`)\"\n      service: box-{port}\n      entryPoints:\n        - websecure\n      tls:\n        certResolver: myresolver\n",
            port = port,
            domain = domain
        );
        traefik_config.push_str(&router_block);
    }

    traefik_config.push_str("  services:\n");
    for port in &used_ports {
        let service_block = format!(
            "    box-{port}:\n      loadBalancer:\n        servers:\n          - url: \"http://ssh_reverse_proxy:{port}\"\n",
            port = port
        );
        traefik_config.push_str(&service_block);
    }

    traefik_config
        .push_str("  middlewares:\n    redirect:\n      redirectScheme:\n        scheme: https\n");
    Ok(traefik_config)
}

/// Endpoint for retrieving Traefik configuration.
#[get("/api/traefik")]
pub async fn get_traefik_config(pool: web::Data<Pool>) -> HttpResponse {
    match generate_traefik_config(pool).await {
        Ok(config) => HttpResponse::Ok().content_type("text/yaml").body(config),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

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
        Ok(_) => (),
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

///
/// Traefik API
/// this route will be used to get the traefik configuration for deferent ports atributed to the mac address
/// he retrun a yaml file with the configuration for traefik
///
/// # Example
/// ```yaml
/// https:
///   routers:
///     box-1:
///       rule: "Host(`box-1.proxy.ski-sync.com`)"
///       service: box-1
///       entryPoints:
///         - secureweb
///   services:
///     box-1:
///       loadBalancer:
///         servers:
///           - url: "http://localhost:8000"
///   middlewares:
///     redirect:
///       redirectScheme:
///         scheme: https
/// ```
///
#[get("/api/traefik")]
pub async fn get_traefik(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let used_ports = match get_used_ports(pool.clone()) {
        Ok(used_ports) => used_ports,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError()
                .body(format!("Error getting used ports: {}", e)))
        }
    };

    let mut traefik_config = String::from("https:\n");
    traefik_config.push_str("  routers:\n");
    for port in &used_ports {
        traefik_config.push_str(&format!("    box-{}:\n", port));
        traefik_config.push_str(&format!(
            "      rule: \"Host(`box-{}.proxy.ski-sync.com`)\"\n",
            port
        ));
        traefik_config.push_str(&format!("      service: box-{}\n", port));
        traefik_config.push_str("      entryPoints:\n");
        traefik_config.push_str("        - secureweb\n");
        // cert resolver
        traefik_config.push_str("      tls:\n");
        traefik_config.push_str("        certResolver: myresolver\n");
    }
    traefik_config.push_str("  services:\n");
    for port in &used_ports {
        traefik_config.push_str(&format!("    box-{}:\n", port));
        traefik_config.push_str("      loadBalancer:\n");
        traefik_config.push_str("        servers:\n");
        traefik_config.push_str(&format!("          - url: \"http://localhost:{}\"\n", port));
    }
    traefik_config.push_str("  middlewares:\n");
    traefik_config.push_str("    redirect:\n");
    traefik_config.push_str("      redirectScheme:\n");
    traefik_config.push_str("        scheme: https\n");

    Ok(HttpResponse::Ok().body(traefik_config))
}

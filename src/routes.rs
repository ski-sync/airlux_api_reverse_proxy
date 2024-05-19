use actix_web::{get, web, Error, HttpResponse};

use crate::{
    db::{
        get_ports_by_mac, get_traefik_dynamic_config, get_used_ports, insert_mac_address,
        insert_ports, Pool,
    }, errors::{ApiError,  ApiResult}, types::{Protocol, Register}
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

/// Generates the YAML configuration for Traefik based on the ports and devices from the database.
///
/// # Arguments
/// * `pool` - A database connection pool.
///
/// # Returns
///
/// * A string containing the YAML configuration for Traefik.
///
pub async fn generate_traefik_config(pool: web::Data<Pool>) -> ApiResult<String> {
    // env DOMAIN_NAME
    let domain_name = match std::env::var("DOMAIN_NAME") {
        Ok(domain_name) => domain_name,
        Err(_) => return Err(ApiError::InternalServerError("DOMAIN_NAME not set".to_string())),
    };

    // Fetch the dynamic configuration
    let dynamic_config = match get_traefik_dynamic_config(pool).await {
        Ok(config) => config,
        Err(_) => {
            return Ok("http:\n  routers:\n  services:\n  middlewares:\n".to_string());
        }
    };

    // http config for each device
    let mut traefik_config = String::from("http:\n  routers:\n");
    for device in &dynamic_config.devices {
        for port in &device.network_ports {
            if port.protocol == Protocol::Http || port.protocol == Protocol::Https {
                let domain = format!("{}.{}.{}", port.port, device.mac_address, domain_name);
                let router_block = format!(
                    "    router-{port}:\n      rule: \"Host(`{domain}`)\"\n      service: service-{port}\n      entryPoints:\n        - websecure\n      tls:\n        certResolver: myresolver\n",
                    port = port.port,
                    domain = domain
                );
                traefik_config.push_str(&router_block);
            }
        }
    }

    // http service config for each device
    traefik_config.push_str("  services:\n");
    for device in &dynamic_config.devices {
        for port in &device.network_ports {
            if port.protocol == Protocol::Http || port.protocol == Protocol::Https {
                let service_block = format!(
                    "    service-{port}:\n      loadBalancer:\n        servers:\n          - url: \"https://ssh_reverse_proxy:{port}\"\n",
                    port = port.port
                );
                traefik_config.push_str(&service_block);
            }
        }
    }

    // Add fixed middleware configuration
    traefik_config
        .push_str("  middlewares:\n    redirect:\n      redirectScheme:\n        scheme: https\n");

    // tcp config for each device
    traefik_config.push_str("tcp:\n  routers:\n");
    for device in &dynamic_config.devices {
        for port in &device.network_ports {
            if port.protocol == Protocol::Tcp {
                let domain = format!("{}.{}.{}", port.port, device.mac_address, domain_name);
                let router_block = format!(
                    "    router-{port}:\n      rule: \"HostSNI(`{domain}`)\"\n      service: service-{port}\n      entryPoints:\n        - websecure\n",
                    port = port.port,
                    domain = domain,
                );
                traefik_config.push_str(&router_block);
            }
        }
    }

    // tcp service config for each device
    traefik_config.push_str("  services:\n");
    for device in &dynamic_config.devices {
        for port in &device.network_ports {
            if port.protocol == Protocol::Tcp {
                let service_block = format!(
                    "    service-{port}:\n      loadBalancer:\n        servers:\n          - address: \"ssh_reverse_proxy:{port}\"\n            tls: true\n",
                    port = port.port
                );
                traefik_config.push_str(&service_block);
            }
        }
    }

    // udp config for each device
    traefik_config.push_str("udp:\n  routers:\n");
    for device in &dynamic_config.devices {
        for port in &device.network_ports {
            if port.protocol == Protocol::Udp {
                let domain = format!("{}.{}.{}", port.port, device.mac_address, domain_name);
                let router_block = format!(
                    "    router-{port}:\n      rule: \"HostSNI(`{domain}`)\"\n      service: service-{port}\n      entryPoints:\n        - websecure\n",
                    port = port.port,
                    domain = domain,
                );
                traefik_config.push_str(&router_block);
            }
        }
    }

    // udp service config for each device
    traefik_config.push_str("  services:\n");
    for device in &dynamic_config.devices {
        for port in &device.network_ports {
            if port.protocol == Protocol::Udp {
                let service_block = format!(
                    "    service-{port}:\n      loadBalancer:\n        servers:\n          - address: \"ssh_reverse_proxy:{port}\"\n            tls: true\n",
                    port = port.port
                );
                traefik_config.push_str(&service_block);
            }
        }
    }

    // Return the generated configuration
    Ok(traefik_config)
}

/// Endpoint for retrieving Traefik configuration.
#[get("/api/traefik")]
pub async fn get_traefik_config(pool: web::Data<Pool>) -> ApiResult<HttpResponse> {
    let config = generate_traefik_config(pool).await?;
    Ok(HttpResponse::Ok().content_type("text/yaml").body(config))
}

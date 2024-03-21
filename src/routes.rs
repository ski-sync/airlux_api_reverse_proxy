use actix_web::{get, web, Error, HttpResponse};

use crate::{
    db::{get_ports_by_mac, get_used_ports, insert_mac_address, insert_ports, Pool},
    port::{Port, Register},
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
pub async fn api_get(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let used_ports = match get_used_ports(pool.clone()) {
        Ok(used_ports) => used_ports,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError()
                .body(format!("Error getting used ports: {}", e)))
        }
    };
    Ok(HttpResponse::Ok().json(used_ports))
}

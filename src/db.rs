pub mod models;
pub mod schema;

use std::collections::BTreeMap;
use std::str::FromStr;

use crate::types::{DeviceConfig, NetworkPortConfig, Port, Protocol, TraefikDynamicConfig};
use actix_web::web;
use diesel::OptionalExtension;
use diesel::{
    r2d2::{self, ConnectionManager},
    result::Error,
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};

pub type DatabaseConnection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DatabaseConnection>>;

pub fn get_used_ports(pool: web::Data<Pool>) -> Result<Vec<u32>, Error> {
    let mut conn = pool.get().unwrap();
    let ports_from_db = schema::ports::table
        .select(schema::ports::port)
        .load::<i32>(&mut conn)
        .unwrap();
    Ok(ports_from_db.into_iter().map(|x| x as u32).collect())
}

pub fn get_ports_by_mac(pool: web::Data<Pool>, address_mac: String) -> Result<Vec<u32>, Error> {
    let mut conn = pool.get().unwrap();

    let mac_id_result: Option<i32> = schema::mac_addresses::table
        .filter(schema::mac_addresses::address_mac.eq(&address_mac))
        .select(schema::mac_addresses::id)
        .first(&mut conn)
        .optional()?;

    let mac_id = match mac_id_result {
        Some(id) => id,
        None => return Ok(Vec::new()),
    };

    let result = schema::ports::table
        .filter(schema::ports::mac_id.eq(mac_id))
        .select(schema::ports::port)
        .load::<i32>(&mut conn)
        .unwrap();

    Ok(result.into_iter().map(|x| x as u32).collect())
}

pub fn insert_mac_address(
    pool: web::Data<Pool>,
    address_mac: String,
    ssh_key: String,
) -> Result<(), Error> {
    let mut conn = pool.get().unwrap();
    let new_mac = models::MacAddresses {
        address_mac,
        ssh_key,
    };
    diesel::insert_into(schema::mac_addresses::table)
        .values(&new_mac)
        .execute(&mut conn)
        .map(|_| ())
}

pub fn insert_ports(
    pool: web::Data<Pool>,
    address_mac: String,
    ports: &Vec<Port>,
) -> Result<(), Error> {
    let mut conn = pool.get().unwrap();

    let mac_id: Option<i32> = schema::mac_addresses::table
        .filter(schema::mac_addresses::address_mac.eq(&address_mac))
        .select(schema::mac_addresses::id)
        .first(&mut conn)
        .optional()?;

    if mac_id.is_none() {
        return Err(Error::NotFound);
    }

    for port in ports {
        let new_port = models::Ports {
            mac_id: mac_id.unwrap(),
            port: port.port as i32,
            protocol: port.protocol.to_string(),
            created: false,
        };
        diesel::insert_into(schema::ports::table)
            .values(new_port)
            .execute(&mut conn)?;
    }
    Ok(())
}

pub async fn get_traefik_dynamic_config(
    pool: web::Data<Pool>,
) -> Result<TraefikDynamicConfig, Error> {
    let mut conn = pool.get().map_err(|_| Error::NotFound)?;

    let results = schema::mac_addresses::table
        .inner_join(schema::ports::table)
        .select((
            schema::mac_addresses::address_mac,
            schema::ports::port,
            schema::ports::protocol,
        ))
        .order(schema::mac_addresses::address_mac::asc(
            schema::mac_addresses::address_mac,
        ))
        .load::<(String, i32, String)>(&mut conn)?;

    let mut device_map: BTreeMap<String, Vec<NetworkPortConfig>> = BTreeMap::new();
    for (mac, port, prot) in results {
        let protocol = Protocol::from_str(&prot).map_err(|_| Error::NotFound)?;
        device_map.entry(mac).or_default().push(NetworkPortConfig {
            port: port as u32,
            protocol,
        });
    }

    let device_configs = device_map
        .into_iter()
        .map(|(mac, ports)| DeviceConfig {
            mac_address: mac,
            network_ports: ports,
        })
        .collect();

    Ok(TraefikDynamicConfig {
        devices: device_configs,
    })
}

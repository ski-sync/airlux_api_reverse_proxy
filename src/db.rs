pub mod models;
pub mod schema;

use actix_web::web;
use diesel::{
    r2d2::{self, ConnectionManager},
    result::Error,
    ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection,
};

use crate::port::Port;

pub type DatabaseConnection = SqliteConnection;
pub type Pool = r2d2::Pool<ConnectionManager<DatabaseConnection>>;

pub fn get_used_ports(pool: web::Data<Pool>) -> Result<Vec<u32>, Error> {
    let mut conn = pool.get().unwrap();
    let ports_from_db = schema::ports::table
        .select(schema::ports::port)
        .load::<i32>(&mut conn)
        .unwrap();
    // let ports_form_server:
    Ok(ports_from_db.into_iter().map(|x| x as u32).collect())
}

pub fn get_ports_by_mac(pool: web::Data<Pool>, address_mac: String) -> Result<Vec<u32>, Error> {
    let mut conn = pool.get().unwrap();
    let mac_id = schema::mac_addresses::table
        .filter(schema::mac_addresses::address_mac.eq(&address_mac))
        .select(schema::mac_addresses::id)
        .first::<Option<i32>>(&mut conn)
        .unwrap();
    let result = schema::ports::table
        .filter(schema::ports::mac_id.eq(mac_id.unwrap()))
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
    let mac_id = schema::mac_addresses::table
        .filter(schema::mac_addresses::address_mac.eq(&address_mac))
        .select(schema::mac_addresses::id)
        .first::<Option<i32>>(&mut conn)
        .unwrap();
    for port in ports {
        let new_port = models::Ports {
            mac_id: mac_id.unwrap(),
            port: port.port as i32,
            protocol: port.protocol.to_string(),
            created: false,
        };
        diesel::insert_into(schema::ports::table)
            .values(new_port) // Remove the reference operator '&'
            .execute(&mut conn)
            .unwrap();
    }
    Ok(())
}

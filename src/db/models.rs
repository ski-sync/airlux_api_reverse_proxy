use core::fmt;
use std::fmt::Display;
use super::schema::mac_addresses;
use super::schema::ports;




#[derive(Queryable, Insertable)]
#[table_name = "mac_addresses"]
pub struct Mac_addresses {
    pub address_mac: String,
    pub ssh_key: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "ports"]
pub struct Ports {
    pub mac_id: i32,
    pub port: i32,
    pub protocol: String,
    pub created: bool,
}
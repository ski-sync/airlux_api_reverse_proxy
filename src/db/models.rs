use super::schema::mac_addresses;
use super::schema::ports;

#[derive(Queryable, Insertable)]
#[diesel(table_name = mac_addresses)]
pub struct MacAddresses {
    pub address_mac: String,
    pub ssh_key: String,
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = ports)]
pub struct Ports {
    pub mac_id: i32,
    pub port: i32,
    pub protocol: String,
    pub created: bool,
}

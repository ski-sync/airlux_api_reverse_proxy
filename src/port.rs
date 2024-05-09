use core::fmt;
use std::fmt::Display;

const START_PORT: u32 = 8000;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Port {
    pub port: u32,
    pub protocol: Protocol,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Register {
    pub address_mac: String,
    pub ports: Ports,
    pub ssh_key: String,
}

impl Register {
    pub fn get_ports(&mut self, used_ports: Vec<u32>) {
        let mut used_ports = used_ports;
        for port in self.ports.0.iter_mut() {
            let mut find_next_port = START_PORT;
            while used_ports.contains(&find_next_port) {
                find_next_port += 1;
            }
            port.port = find_next_port;
            used_ports.push(find_next_port);
        }
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Ports(pub Vec<Port>);

impl Ports {
    pub fn get_ports(&self) -> Vec<u32> {
        self.0.iter().map(|x| x.port).collect()
    }

    pub fn get(&self) -> Vec<Port> {
        self.0.clone()
    }
}

#[derive(serde::Serialize)]
struct ResponseRegister {
    pub ports: Vec<u32>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub enum Protocol {
    Tcp,
    Udp,
    Http,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Protocol::Tcp => write!(f, "Tcp"),
            Protocol::Udp => write!(f, "Udp"),
            Protocol::Http => write!(f, "Http"),
        }
    }
}

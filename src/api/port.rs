use core::fmt;
use reqwest::blocking::Client;
use std::env;
use std::fmt::Display;

const START_PORT: u32 = 8000;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Port {
    pub port: u32,
    pub protocol: Protocol,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Register {
    pub address_mac: String,
    pub ports: Ports,
    pub ssh_key: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct RegisterAuthorizeKey {
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

    ///
    /// this function save ssh_key in authorized_keys file
    /// for this he use authorize_key microservice to save ssh_key
    ///
    pub fn save_ssh_key(&self) {
        let client = Client::new();
        let url = format!(
            "http://{}:{}/register",
            env::var("AUTHORIZE_KEY_HOST").unwrap(),
            env::var("AUTHORIZE_KEY_PORT").unwrap()
        );

        let response = client
            .post(&url)
            .json(&RegisterAuthorizeKey {
                ssh_key: self.ssh_key.clone(),
            })
            .send()
            .expect("Error sending request");

        if response.status().is_success() {
            println!("Success");
        } else {
            println!("Error: {:?}", response);
            // print request body
            println!("{:?}", response.text());
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
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

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum Protocol {
    Tcp,
    Http,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Protocol::Tcp => write!(f, "Tcp"),
            Protocol::Http => write!(f, "Http"),
        }
    }
}

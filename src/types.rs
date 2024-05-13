use core::fmt;
use std::fmt::Display;

const START_PORT: u32 = 8000;

///
/// Register struct
///
#[derive(serde::Deserialize, Debug, Clone)]
pub struct Register {
    pub address_mac: String,
    pub ports: Ports,
    pub ssh_key: String,
}

impl Register {
    ///
    /// Get ports for the register
    ///
    /// # Arguments
    ///
    /// * `self` - The object instance
    /// * `used_ports` - A vector of used ports
    ///
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

///
/// Ports struct
///
#[derive(serde::Deserialize, Debug, Clone)]
pub struct Ports(pub Vec<Port>);

impl Ports {
    ///
    /// Get ports
    ///
    /// # Arguments
    ///
    /// * `self` - The object instance
    ///
    /// # Returns
    ///
    /// * A vector of ports as u32
    ///
    pub fn get_ports(&self) -> Vec<u32> {
        self.0.iter().map(|x| x.port).collect()
    }

    ///
    /// Get ports
    ///
    /// # Arguments
    ///
    /// * `self` - The object instance
    ///
    /// # Returns
    ///
    /// * A vector of ports
    ///
    pub fn get(&self) -> Vec<Port> {
        self.0.clone()
    }
}

///
/// Port struct
///
#[derive(serde::Deserialize, Debug, Clone)]
pub struct Port {
    pub port: u32,
    pub protocol: Protocol,
}

///
/// Protocol enum
///
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum Protocol {
    Tcp,
    Udp,
    Http,
    Https,
}

///
/// Implement the Display trait for the Protocol enum
/// This allows formatting a Protocol enum as a string
/// when printing or converting to a string
///
impl Display for Protocol {
    ///
    /// Format the protocol
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Protocol::Tcp => write!(f, "Tcp"),
            Protocol::Udp => write!(f, "Udp"),
            Protocol::Http => write!(f, "Http"),
            Protocol::Https => write!(f, "Https"),
        }
    }
}

///
/// Implement the FromStr trait for the Protocol enum
/// This allows parsing a string into a Protocol enum
///
impl std::str::FromStr for Protocol {
    type Err = String;

    ///
    /// Parse the protocol
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Tcp" => Ok(Protocol::Tcp),
            "Udp" => Ok(Protocol::Udp),
            "Http" => Ok(Protocol::Http),
            "Https" => Ok(Protocol::Https),
            _ => Err(format!("Invalid protocol: {}", s)),
        }
    }
}

/// Represents a dynamic configuration for Traefik, mapping devices to their respective network ports.
///
/// This structure facilitates generating responses for Traefik's dynamic configuration system,
/// enabling traffic routing based on rules associated with specific devices' MAC addresses and their ports.
///
/// # Fields
/// - `devices`: A vector of `DeviceConfig` instances, each defining a unique device by its MAC address and assigned network ports.
///
/// # Example
///
/// ```rust
/// use crate::types::TraefikDynamicConfig;
///
/// let traefik_config = TraefikDynamicConfig {
///     devices: vec![
///         DeviceConfig {
///             mac_address: "00:00:00:00:00:00",
///             network_ports: vec![
///                 NetworkPortConfig {
///                     port: 8000,
///                     protocol: "Tcp",
///                 },
///                 NetworkPortConfig {
///                     port: 8001,
///                     protocol: "Tcp",
///                 }
///             ],
///         }
///     ],
/// };
/// ```
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct TraefikDynamicConfig {
    pub devices: Vec<DeviceConfig>,
}

/// Configuration for a device, specifying its MAC address and the network ports configurations.
///
/// # Fields
/// - `mac_address`: The MAC address of the device.
/// - `network_ports`: A list of `NetworkPortConfig` detailing each port and its associated protocol.
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct DeviceConfig {
    pub mac_address: String,
    pub network_ports: Vec<NetworkPortConfig>,
}

/// Details a specific port and the protocol associated with it.
///
/// # Fields
/// - `port`: The network port number.
/// - `protocol`: The network protocol used, typically "Tcp" or "Udp".
#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct NetworkPortConfig {
    pub port: u32,
    pub protocol: Protocol,
}

pub struct NetworkInterface {
    pub ipv4: String,
    pub ipv6: String,
    pub mac: String,
    pub name: String,
    pub status: bool,
    pub dns: String,
    pub gateway: String,
}

pub struct Network {
    pub upload: f64,
    pub download: f64,
    pub latency: f64,
    pub interfaces: Vec<NetworkInterface>,
}

impl Network {
    pub fn default() -> Self {
        Self {
            upload: 0.0,        // Default to 0 Mbps upload speed
            download: 0.0,      // Default to 0 Mbps download speed
            latency: 0.0,       // Default to 0 ms latency
            interfaces: vec![], // Default to no network interfaces
        }
    }
}

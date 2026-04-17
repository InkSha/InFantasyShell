const BIT: isize = 1;
const BYTE: isize = 8 * BIT;
const KILOBYTE: isize = 1024 * BYTE;
const MEGABYTE: isize = 1024 * KILOBYTE;
const GIGABYTE: isize = 1024 * MEGABYTE;
const TERABYTE: isize = 1024 * GIGABYTE;

pub enum DataSize {
    Bit = BIT,
    Byte = BYTE,
    Kilobyte = KILOBYTE,
    Megabyte = MEGABYTE,
    Gigabyte = GIGABYTE,
    Terabyte = TERABYTE,
}

fn format_size(size: isize) -> String {
    if size >= TERABYTE {
        format!("{:.2} TB", size as f64 / TERABYTE as f64)
    } else if size >= GIGABYTE {
        format!("{:.2} GB", size as f64 / GIGABYTE as f64)
    } else if size >= MEGABYTE {
        format!("{:.2} MB", size as f64 / MEGABYTE as f64)
    } else if size >= KILOBYTE {
        format!("{:.2} KB", size as f64 / KILOBYTE as f64)
    } else {
        format!("{} B", size)
    }
}

pub struct CPU {
    pub core: usize,
    pub thread: usize,
    pub frequency: f64,
    pub usage: f64,
}

pub struct Memory {
    pub total: usize,
    pub used: usize,
    pub free: usize,
}

pub struct Storage {
    pub total: usize,
    pub used: usize,
    pub free: usize,
}

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

pub struct System {
    pub cpu: CPU,
    pub memory: Memory,
    pub storage: Storage,
    pub network: Network,
}

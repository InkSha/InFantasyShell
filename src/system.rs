pub mod bit;
pub mod cpu;
pub mod error;
pub mod memory;
pub mod network;
pub mod permission;
pub mod storage;

pub struct System {
    pub cpu: cpu::CPU,
    pub memory: memory::Memory,
    pub storage: storage::Storage,
    pub network: network::Network,
}

impl System {
    pub fn default() -> Self {
        Self {
            cpu: cpu::CPU::default(),
            memory: memory::Memory::defualt(),
            storage: storage::Storage::default(),
            network: network::Network::default(),
        }
    }
}

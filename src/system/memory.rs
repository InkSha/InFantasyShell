use crate::system::bit::DataSize;

pub struct Memory {
    pub total: DataSize,
    pub used: DataSize,
}

impl Memory {
    pub fn defualt() -> Self {
        Self {
            total: DataSize::Gigabyte(2),  // 2 GB total memory
            used: DataSize::Megabyte(500), // 500 MB used memory
        }
    }
}

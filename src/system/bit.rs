const BIT: usize = 1;
const BYTE: usize = 8 * BIT;
const KILOBYTE: usize = 1024 * BYTE;
const MEGABYTE: usize = 1024 * KILOBYTE;
const GIGABYTE: usize = 1024 * MEGABYTE;
const TERABYTE: usize = 1024 * GIGABYTE;

#[derive(Debug, Clone)]
pub enum DataSize {
    Bit(usize),
    Byte(usize),
    Kilobyte(usize),
    Megabyte(usize),
    Gigabyte(usize),
    Terabyte(usize),
}

impl DataSize {
    pub fn to_bytes(&self) -> usize {
        match self {
            DataSize::Bit(size) => BIT * size,
            DataSize::Byte(size) => BYTE * size,
            DataSize::Kilobyte(size) => KILOBYTE * size,
            DataSize::Megabyte(size) => MEGABYTE * size,
            DataSize::Gigabyte(size) => GIGABYTE * size,
            DataSize::Terabyte(size) => TERABYTE * size,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            DataSize::Bit(size) => format!("{} bits", size),
            DataSize::Byte(size) => format!("{} bytes", size),
            DataSize::Kilobyte(size) => format!("{} KB", size),
            DataSize::Megabyte(size) => format!("{} MB", size),
            DataSize::Gigabyte(size) => format!("{} GB", size),
            DataSize::Terabyte(size) => format!("{} TB", size),
        }
    }
}

impl Into<usize> for DataSize {
    fn into(self) -> usize {
        self.to_bytes()
    }
}

impl From<usize> for DataSize {
    fn from(size: usize) -> Self {
        if size >= TERABYTE {
            DataSize::Terabyte(size / TERABYTE)
        } else if size >= GIGABYTE {
            DataSize::Gigabyte(size / GIGABYTE)
        } else if size >= MEGABYTE {
            DataSize::Megabyte(size / MEGABYTE)
        } else if size >= KILOBYTE {
            DataSize::Kilobyte(size / KILOBYTE)
        } else if size >= BYTE {
            DataSize::Byte(size / BYTE)
        } else {
            DataSize::Bit(size)
        }
    }
}

impl std::ops::Add for DataSize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        DataSize::from(self.to_bytes() + rhs.to_bytes())
    }
}

impl std::ops::Sub for DataSize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        DataSize::from(self.to_bytes() - rhs.to_bytes())
    }
}

impl std::ops::Mul for DataSize {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        DataSize::from(self.to_bytes() * rhs.to_bytes())
    }
}

impl std::ops::Div for DataSize {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        DataSize::from(self.to_bytes() / rhs.to_bytes())
    }
}

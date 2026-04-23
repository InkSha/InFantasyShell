use crate::system::error::FsError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PermBits {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl PermBits {
    pub const fn new(read: bool, write: bool, execute: bool) -> Self {
        Self {
            read,
            write,
            execute,
        }
    }

    fn to_octal_digit(&self) -> u8 {
        (u8::from(self.read) * 4) + (u8::from(self.write) * 2) + u8::from(self.execute)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permissions {
    pub owner: PermBits,
    pub group: PermBits,
    pub other: PermBits,
}

impl Permissions {
    pub const fn file_default() -> Self {
        Self {
            owner: PermBits::new(true, true, false),
            group: PermBits::new(true, false, false),
            other: PermBits::new(true, false, false),
        }
    }

    pub const fn directory_default() -> Self {
        Self {
            owner: PermBits::new(true, true, true),
            group: PermBits::new(true, false, true),
            other: PermBits::new(true, false, true),
        }
    }

    pub const fn executable_file() -> Self {
        Self {
            owner: PermBits::new(true, true, true),
            group: PermBits::new(true, false, true),
            other: PermBits::new(true, false, true),
        }
    }

    pub fn from_octal(input: &str) -> Result<Self, FsError> {
        if input.len() != 3 || !input.chars().all(|ch| ('0'..='7').contains(&ch)) {
            return Err(FsError::InvalidPermissions(input.to_string()));
        }

        let mut digits = input.chars();
        let owner = digits
            .next()
            .and_then(|ch| ch.to_digit(8))
            .ok_or_else(|| FsError::InvalidPermissions(input.to_string()))?
            as u8;
        let group = digits
            .next()
            .and_then(|ch| ch.to_digit(8))
            .ok_or_else(|| FsError::InvalidPermissions(input.to_string()))?
            as u8;
        let other = digits
            .next()
            .and_then(|ch| ch.to_digit(8))
            .ok_or_else(|| FsError::InvalidPermissions(input.to_string()))?
            as u8;

        Ok(Self {
            owner: Self::bits_from_digit(owner),
            group: Self::bits_from_digit(group),
            other: Self::bits_from_digit(other),
        })
    }

    fn bits_from_digit(digit: u8) -> PermBits {
        PermBits::new(digit & 4 != 0, digit & 2 != 0, digit & 1 != 0)
    }

    pub fn to_octal_string(&self) -> String {
        format!(
            "{}{}{}",
            self.owner.to_octal_digit(),
            self.group.to_octal_digit(),
            self.other.to_octal_digit()
        )
    }
}

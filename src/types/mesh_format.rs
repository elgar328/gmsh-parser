use crate::parser::Token;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Ascii,
    Binary,
}

impl FileType {
    pub fn to_i32(self) -> i32 {
        match self {
            FileType::Ascii => 0,
            FileType::Binary => 1,
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::Ascii => write!(f, "0 (ASCII)"),
            FileType::Binary => write!(f, "1 (Binary)"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub token: Token,
}

impl Version {
    pub fn new(major: u32, minor: u32, token: Token) -> Self {
        Self {
            major,
            minor,
            token,
        }
    }

    /// Check if this version is supported (only MSH 4.1 is supported)
    pub fn is_supported(&self) -> bool {
        self.major == 4 && self.minor == 1
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[derive(Debug, Clone)]
pub struct MeshFormat {
    pub version: Version,
    pub file_type: FileType,
    pub data_size: i32,
}

impl MeshFormat {
    pub fn new(version: Version, file_type: FileType, data_size: i32) -> Self {
        Self {
            version,
            file_type,
            data_size,
        }
    }
}

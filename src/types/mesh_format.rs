use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Ascii,
    Binary,
}

impl FileType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(FileType::Ascii),
            1 => Some(FileType::Binary),
            _ => None,
        }
    }

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

#[derive(Debug, Clone, PartialEq)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: Option<u32>,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: Option<u32>) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl Version {
    pub fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.len() < 2 || parts.len() > 3 {
            return None;
        }

        let major = parts[0].parse::<u32>().ok()?;
        let minor = parts[1].parse::<u32>().ok()?;
        let patch = if parts.len() == 3 {
            Some(parts[2].parse::<u32>().ok()?)
        } else {
            None
        };

        Some(Version {
            major,
            minor,
            patch,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(patch) = self.patch {
            write!(f, "{}.{}.{}", self.major, self.minor, patch)
        } else {
            write!(f, "{}.{}", self.major, self.minor)
        }
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

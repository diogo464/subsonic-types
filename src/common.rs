use serde::{Deserialize, Serialize};

/// A duration in milliseconds.
/// When used to represent an instant in time, it is relative to the Unix epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Milliseconds(pub u64);

impl Milliseconds {
    pub fn new(milliseconds: u64) -> Self {
        Self(milliseconds)
    }
}

impl From<u64> for Milliseconds {
    fn from(milliseconds: u64) -> Self {
        Self::new(milliseconds)
    }
}

/// A duration in seconds.
/// When used to represent an instant in time, it is relative to the Unix epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Seconds(pub u64);

impl Seconds {
    pub fn new(seconds: u64) -> Self {
        Self(seconds)
    }
}

impl From<u64> for Seconds {
    fn from(seconds: u64) -> Self {
        Self::new(seconds)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MusicFolderId(u32);

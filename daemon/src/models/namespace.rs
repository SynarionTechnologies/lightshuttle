use serde::Deserialize;

/// Logical grouping of resources with associated permissions.
#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
pub struct Namespace {
    /// Human-readable namespace identifier.
    pub name: String,
    /// Permission to perform read operations.
    pub read: bool,
    /// Permission to perform write operations.
    pub write: bool,
}

#[allow(dead_code)]
impl Namespace {
    /// Returns `true` if read operations are permitted.
    pub fn can_read(&self) -> bool {
        self.read
    }

    /// Returns `true` if write operations are permitted.
    pub fn can_write(&self) -> bool {
        self.write
    }
}

//! API version enum.

use std::fmt;

/// The API version used to request a particular endpoint.
#[derive(
    Debug, Clone,
    PartialEq, Eq, PartialOrd, Ord
)]
pub enum Version {
    /// Version 1 (Deprecated)
    V1 = 1,
    /// Version 2 (Current)
    V2
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Version::V1 => "v1",
            Version::V2 => "v2"
        })
    }
}
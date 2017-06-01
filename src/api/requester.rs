//! API `Requester` trait.

use serde::de::DeserializeOwned;

use super::Version;

/// Generic trait used to query the API and return an instance of a `Deserialize`able struct.
pub trait Requester {
    fn request<T>(&self, version: Version, endpoint: &str) -> ::Result<T> where T: DeserializeOwned;
}
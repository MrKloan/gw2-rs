/// API v1 implementation.

use api::prelude::*;
use super::models::*;

pub struct API {
	client: Client
}

impl API {
	
	pub fn new(client: Client) -> Self {
		API {
			client: client
		}
	}
	
	pub fn builder() -> Builder {
		Builder::new(Version::V1)
	}
	
	pub fn build(&self) -> ::Result<Build> {
		self.client.request::<Build>("build")
	}
}
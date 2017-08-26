//! API v1 implementation.

use api::prelude::*;
use super::models::*;
use super::specs::ApiSpecs;

pub struct Api {
	client: Client
}

impl Api {
	
	pub fn new(client: Client) -> Self {
		Api {
			client: client
		}
	}
	
	pub fn builder() -> Builder {
		Builder::new(Version::V1)
	}
}

impl ApiSpecs for Api {
	
	fn build(&self) -> ::Result<Build> {
		self.client.request::<Build>("build")
	}
}
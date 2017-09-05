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
		Builder::new(Version::V2)
	}
}

impl Default for Api {
	fn default() -> Self {
		Api::new(Api::builder().into())
	}
}

impl ApiSpecs for Api {
	
	fn build(&self) -> ::Result<Build> {
		self.client.request::<Build>("build")
	}
	
	fn cats(&self) -> ::Result<Vec<i32>> {
		self.client.request::<Vec<i32>>("cats")
	}
	
	fn cat(&self, id: i32) -> ::Result<Cat> {
		let endpoint = format!("cats/{}", id);
		self.client.request::<Cat>(&endpoint)
	}
	
	fn colors(&self) -> ::Result<Vec<i32>> {
		self.client.request::<Vec<i32>>("colors")
	}
	
	fn color(&self, id: i32) -> ::Result<Color> {
		let endpoint = format!("colors/{}", id);
		self.client.request::<Color>(&endpoint)
	}
	
	fn currencies(&self) -> ::Result<Vec<i32>> {
		self.client.request::<Vec<i32>>("currencies")
	}
	
	fn currency(&self, id: i32) -> ::Result<Currency> {
		let endpoint = format!("currencies/{}", id);
		self.client.request::<Currency>(&endpoint)
	}
	
	fn dungeons(&self) -> ::Result<Vec<String>> {
		self.client.request::<Vec<String>>("dungeons")
	}

	fn dungeon(&self, id: &str) -> ::Result<Dungeon> {
		let endpoint = format!("dungeons/{}", id);
		self.client.request::<Dungeon>(&endpoint)
	}
	
	fn quaggans(&self) -> ::Result<Vec<String>> {
		self.client.request::<Vec<String>>("quaggans")
	}
	
	fn quaggan(&self, id: &str) -> ::Result<Quaggan> {
		let endpoint = format!("quaggans/{}", id);
		self.client.request::<Quaggan>(&endpoint)
	}
}
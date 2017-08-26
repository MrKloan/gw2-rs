//! API v2 specifications trait.

use api::v2::models::*;

pub trait ApiSpecs {
	fn build(&self) -> ::Result<Build>;
	
	fn cats(&self) -> ::Result<Vec<i32>>;
	fn cat(&self, id: i32) -> ::Result<Cat>;
	
	fn colors(&self) -> ::Result<Vec<i32>>;
	fn color(&self, id: i32) -> ::Result<Color>;
}
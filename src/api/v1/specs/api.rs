//! API v1 specifications trait.

use api::v1::models::*;

pub trait ApiSpecs {
	fn build(&self) -> ::Result<Build>;
}
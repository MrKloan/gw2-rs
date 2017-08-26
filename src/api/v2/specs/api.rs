//! API v2 specifications trait.

use api::v2::models::*;

pub trait ApiSpecs {
	fn build(&self) -> ::Result<Build>;
}
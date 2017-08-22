//! Rust wrapper for the [Guild Wars 2](https://www.guildwars2.com/) REST and MumbleLink API.

// Extern crates
#[macro_use]
extern crate error_chain;

extern crate hyper;
extern crate hyper_native_tls;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

// Constants
/// Package name
pub const PACKAGE: &'static str = env!("CARGO_PKG_NAME");
/// Current version
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
/// Package description
pub const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
/// Package authors
pub const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

// Internal modules
mod error;
pub use error::*;

pub mod api;

#[cfg(test)]
mod tests {
	use api::prelude::*;
	
	#[test]
	fn api_default_build() {
		let client: Client = Builder::new(Version::V2).into();
		let build = client.request::<Build>("build");
		
		assert!(build.is_ok());
		assert!(*build.unwrap() > 0);
	}
}
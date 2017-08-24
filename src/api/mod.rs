//! Guild Wars 2 web service bridge.

/// Guild Wars 2 API url
pub const URL: &'static str = "https://api.guildwars2.com";

// Enums
mod version;
mod lang;

// Client
mod requester;
mod client;
mod builder;

// API implementations
pub mod v1;

// Prelude
pub mod prelude;

#[cfg(test)]
mod tests {
	use api::v1::API;
	
	#[test]
	fn api_default_build() {
		let api = API::new(API::builder().into());
		let build = api.build();
		
		assert!(build.is_ok());
		assert!(*build.unwrap() > 0);
	}
}
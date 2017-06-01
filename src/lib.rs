//! Guild Wars 2 API wrapper.

// Extern crates
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
/// Guild Wars 2 API url
pub const API_URL: &'static str = "https://api.guildwars2.com";

// Internal modules
mod error;
pub mod api;

// Public exports
pub use error::*;
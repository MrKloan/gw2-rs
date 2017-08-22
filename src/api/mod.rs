//! Guild Wars 2 web service bridge.

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

/// Guild Wars 2 API url
pub const URL: &'static str = "https://api.guildwars2.com";
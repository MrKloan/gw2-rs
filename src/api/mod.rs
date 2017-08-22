//! Guild Wars 2 web service bridge.

// Enums
mod version;
mod lang;

// Client
mod requester;
mod client;
mod builder;

// Models
mod models;

// Prelude
pub mod prelude;

/// Guild Wars 2 API url
pub const URL: &'static str = "https://api.guildwars2.com";
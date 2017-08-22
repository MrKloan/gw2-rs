/// API v1 Build model: https://api.guildwars2.com/v1/build.json

use std::fmt;
use std::ops::Deref;

#[derive(
	Deserialize,
	Debug, Clone,
	PartialEq, Eq, Ord, PartialOrd
)]
pub struct Build {
	build_id: i32
}

impl Build {
	pub fn new(id: i32) -> Build {
		Build {
			build_id: id
		}
	}
}

impl fmt::Display for Build {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.build_id)
	}
}

impl Deref for Build {
	type Target = i32;
	
	fn deref(&self) -> &i32 {
		&self.build_id
	}
}
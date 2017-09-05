//! API v2 `/cats`: https://wiki.guildwars2.com/wiki/API:2/cats

#[derive(
	Deserialize,
	Debug, Clone,
	PartialEq, Eq, Ord, PartialOrd
)]
pub struct Cat {
	pub id: i32,
	pub hint: String
}
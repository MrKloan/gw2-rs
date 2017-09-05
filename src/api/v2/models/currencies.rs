//! API v2 `/currencies`: https://wiki.guildwars2.com/wiki/API:2/currencies

#[derive(
	Deserialize,
	Debug, Clone,
	PartialEq, Eq, Ord, PartialOrd
)]
pub struct Currency {
	pub id: i32,
	pub name: String,
	pub description: String,
	pub icon: String,
	pub order: i32
}
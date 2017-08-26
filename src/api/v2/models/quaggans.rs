//! API v2 `/quaggans`: https://wiki.guildwars2.com/wiki/API:2/quaggans

#[derive(
	Deserialize,
	Debug, Clone,
	PartialEq, Eq, Ord, PartialOrd
)]
pub struct Quaggan {
	pub id: String,
	pub url: String
}
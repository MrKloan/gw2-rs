//! API v2 `/dungeons`: https://wiki.guildwars2.com/wiki/API:2/dungeons

#[derive(
	Deserialize,
	Debug, Clone,
	PartialEq, Eq, Ord, PartialOrd
)]
pub struct Dungeon {
	id: String,
	paths: Vec<DungeonPath>
}

#[derive(
	Deserialize,
	Debug, Clone,
	PartialEq, Eq, Ord, PartialOrd
)]
pub struct DungeonPath {
	pub id: String
}
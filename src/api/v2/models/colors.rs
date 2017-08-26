//! API v2 `/colors`: https://wiki.guildwars2.com/wiki/API:2/colors

#[derive(
	Deserialize,
	Debug, Clone
)]
pub struct Color {
	pub id: i32,
	pub name: String,
	pub base_rgb: [i32; 3],
	pub cloth: Material,
	pub leather: Material,
	pub metal: Material,
	#[serde(default)]
	pub item: i32,
	pub categories: Vec<String>
}

#[derive(
	Deserialize,
	Debug, Clone
)]
pub struct Material {
	pub brightness: i32,
	pub contrast: f32,
	pub hue: i32,
	pub saturation: f32,
	pub lightness: f32,
	pub rgb: [i32; 3]
}
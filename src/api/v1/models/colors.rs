//! Colors models.

pub struct

pub struct Color {
	pub name: String,
	pub base_rgb: [i32; 3],
	pub cloth: Material,
	pub leather: Material,
	pub metal: Material,
	pub item: i32,
	pub categories: Vec<String>
}

pub struct Material {
	pub brightness: i32,
	pub contrast: f32,
	pub hue: i32,
	pub saturation: f32,
	pub lightness: f32,
	pub rgb: [i32; 3]
}
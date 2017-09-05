use libc::wchar_t;

#[derive(Copy, Clone)]
pub struct Position {
	/// The character's position in space.
	pub position: [f32; 3],
	/// A unit vector pointing out of the character's eyes.
	pub front: [f32; 3],
	/// A unit vector pointing out of the top of the character's head.
	pub top: [f32; 3]
}

impl Default for Position {
	fn default() -> Self {
		Position {
			position: [0., 0., 0.],
			front: [0., 0., 1.],
			top: [0., 1., 0.],
		}
	}
}

#[derive(Copy)]
pub struct LinkedMem {
	ui_version: u32,
	ui_tick: u32,
	avatar: Position,
	name: [wchar_t; 256],
	camera: Position,
	identity: [wchar_t; 256],
	context_len: u32,
	context: [u8; 256],
	description: [wchar_t; 2048]
}

impl LinkedMem {
	pub fn ui_version(&self) -> u32 {
		self.ui_version
	}
	
	pub fn ui_tick(&self) -> u32 {
		self.ui_tick
	}
	
	pub fn avatar(&self) -> Position {
		self.avatar.clone()
	}
	
	pub fn name(&self) -> String {
		String::from_utf16_lossy(&self.name)
	}
	
	pub fn camera(&self) -> Position {
		self.camera.clone()
	}
	
	pub fn identity(&self) -> String {
		String::from_utf16_lossy(&self.identity)
	}
	
	pub fn context_len(&self) -> u32 {
		self.context_len
	}
	
	pub fn context(&self) -> String {
		String::from(String::from_utf8_lossy(&self.context))
	}
	
	pub fn description(&self) -> String {
		String::from_utf16_lossy(&self.description)
	}
}

impl Default for LinkedMem {
	fn default() -> Self {
		LinkedMem {
			ui_version: 0,
			ui_tick: 0,
			avatar: Position::default(),
			name: [0; 256],
			camera: Position::default(),
			identity: [0; 256],
			context_len: 0,
			context: [0; 256],
			description: [0; 2048]
		}
	}
}

impl Clone for LinkedMem {
	fn clone(&self) -> Self {
		*self
	}
}
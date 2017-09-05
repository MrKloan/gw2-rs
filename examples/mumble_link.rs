extern crate gw2;

use std::{thread, time};
use gw2::link::GW2;

fn main() {
	let mut gw2 = GW2::new().expect("Unable to link to Guild Wars 2");
	
	loop {
		if let Some(link) = gw2.tick() {
			println!("UI version: {}", link.ui_version());
			println!("UI tick: {}", link.ui_tick());
			println!("Avatar: Position[{},{},{}], Front[{},{},{}], Top[{},{},{}]", link.avatar().position[0], link.avatar().position[1], link.avatar().position[2], link.avatar().front[0], link.avatar().front[1], link.avatar().front[2], link.avatar().top[0], link.avatar().top[1], link.avatar().top[2]);
			println!("Name: {}", link.name());
			println!("Camera: Position[{},{},{}], Front[{},{},{}], Top[{},{},{}]", link.camera().position[0], link.camera().position[1], link.camera().position[2], link.camera().front[0], link.camera().front[1], link.camera().front[2], link.camera().top[0], link.camera().top[1], link.camera().top[2]);
			println!("Identity: {}", link.identity());
			println!("Context len: {}", link.context_len());
			println!("Context: {}", link.context());
			println!("Description: {}", link.description());
		}
			else {
				println!("No new data...");
			}
		
		thread::sleep(time::Duration::from_millis(2000));
	}
}
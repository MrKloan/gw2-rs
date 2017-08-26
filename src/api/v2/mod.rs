//! GW2 REST API v1 implementation

mod models;
mod specs;
mod api;

pub mod prelude;

#[cfg(test)]
mod tests {
	use api::v2::prelude::*;
	
	#[test]
	fn build() {
		let api = Api::default();
		let build = api.build();
		
		assert!(build.is_ok());
		assert!(*build.unwrap() > 0);
	}
	
	#[test]
	fn cats() {
		let api = Api::default();
		let cats = api.cats();
		
		assert!(cats.is_ok());
		assert_eq!(cats.unwrap().len(), 32);
	}
	
	#[test]
	fn cat() {
		let api = Api::default();
		let cat = api.cat(1);
		
		assert!(cat.is_ok());
		assert_eq!(cat.unwrap().hint, "chicken");
	}
	
	#[test]
	fn colors() {
		let api = Api::default();
		let colors = api.colors();
		
		assert!(colors.is_ok());
		assert_eq!(colors.unwrap().len(), 531);
	}
	
	#[test]
	fn color() {
		let api = Api::default();
		let color = api.color(1);
		
		assert!(color.is_ok());
		assert_eq!(color.unwrap().name, "Dye Remover");
	}
}
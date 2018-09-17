use ggez;
use ggez::graphics::{Point2, Vector2};
use specs;
use specs::Builder;

use scene::Sprite;
use components::*;

pub struct World {
	pub specs_world: specs::World,
}

impl World{
	fn register_components(&mut self) {
		// register components using self.specs_world.register::<Component>();
		self.specs_world.register::<Position>();
		self.specs_world.register::<Motion>();
		self.specs_world.register::<Size>();
		self.specs_world.register::<Mass>();
		self.specs_world.register::<Sprite>();
	}

	pub fn new(ctx: &mut ggez::Context) -> Self {
		let w = specs::World::new();

		let mut the_world = Self {
			specs_world: w,
		};

		the_world.register_components();

		the_world
			.specs_world
			.create_entity() 
			.with(Position(Point2::new(0.0,0.0)))
			.build();

		the_world
	}
}
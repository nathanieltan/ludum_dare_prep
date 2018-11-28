use ggez;
use ggez::graphics::{Point2, Vector2};
use specs;
use specs::Builder;

use warmy;
use warmy::SimpleKey;
use std::path;

use components::*;

pub struct World {
	pub assets: warmy::Store<ggez::Context, SimpleKey>,
	pub specs_world: specs::World,
}

impl World{
	fn register_components(&mut self) {
		// register components using self.specs_world.register::<Component>();
		self.specs_world.register::<Position>();
		self.specs_world.register::<Motion>();
		self.specs_world.register::<Size>();
		self.specs_world.register::<Mass>();
	}

	pub fn new(ctx: &mut ggez::Context, resource_dir: Option<path::PathBuf>) -> Self {
    	// ggez assumes absolute paths and warmy assumes system-absolute paths, 
		// so we have warmy look in the specified resource directory 
		// ($CARGO_DIR/resources) or the ggez default resource dir
		let resource_pathbuf: path::PathBuf = match resource_dir {
			Some(s) => s,
			None => ctx.filesystem.get_resources_dir().to_owned(),
		};

		let opt = warmy::StoreOpt::default().set_root(resource_pathbuf);
		let store = warmy::Store::new(opt)
			.expect("Could not create asset store? Does the directory exist?");

		let w = specs::World::new();

		let mut the_world = Self {
			assets: store,
			specs_world: w,
		};

		the_world.register_components();

		the_world.register_components();

		the_world
	}
}
extern crate ggez;
extern crate ggez_goodies;
extern crate rand;
extern crate warmy;
extern crate specs;
extern crate nalgebra;
#[macro_use]
extern crate log;
extern crate ncollide2d;
#[macro_use]
extern crate failure;


use ggez::conf;
use ggez::event::*;
use ggez::{Context, ContextBuilder, GameResult, timer, graphics, event};
use std::path;
//use ggez::graphics::{self, Point2, Rect};

#[macro_use]
extern crate specs_derive;

//Modules for ECS and other content
mod scenes;
mod components;
mod systems;
mod world;

// Utility Modules
mod input;
mod error;
mod util;
mod resources;

struct Game{
	scenes: scenes::FSceneStack,
	input_binding: input::InputBinding,
}

impl Game {
	fn new(resource_dir: Option<path::PathBuf>, ctx: &mut Context) -> Self{
		let mut world = world::World::new(ctx, resource_dir.clone());

		// Sets up scenestack and inital scenes
		let mut scenestack = scenes::FSceneStack::new(ctx, world);
		let level_scene = scenes::level::LevelScene::new(ctx, &mut scenestack.world)
			.expect("Could not create initial scene?!");
		scenestack.push(Box::new(level_scene));

		Game { 
			scenes: scenestack,
			input_binding: input::create_input_binding(),
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		const DESIRED_FPS: u32 = 60; 		// TODO Move to config location/file
		while timer::check_update_time(ctx, DESIRED_FPS){
			self.scenes.update();
		}
		self.scenes.world.assets.sync(ctx);
		self.scenes.world.input.update(1.0/ 60.0);

		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		graphics::clear(ctx);
		self.scenes.draw(ctx);
		graphics::present(ctx);
		Ok(())
	}

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.world.input.update_effect(ev, true);
            self.scenes.input(ev, true);
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        if let Some(ev) = self.input_binding.resolve(keycode) {
            self.scenes.world.input.update_effect(ev, false);
            self.scenes.input(ev, false);
        }
}
}

fn main() {
    let mut cb = ContextBuilder::new("ldPrep", "sourdile")
        .window_setup(conf::WindowSetup::default().title("Ludum Dare Prep"))
        .window_mode(conf::WindowMode::default().dimensions(800, 600));

    // We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    // we we look in the cargo project for files.
    // And save it so we can feed there result into warmy

    let cargo_path: Option<path::PathBuf> = option_env!("CARGO_MANIFEST_DIR").map(|env_path| {
        let mut res_path = path::PathBuf::from(env_path);
        res_path.push("resources");
        res_path
    });
    // If we have such a path then add it to the ctx builder too
    // (modifying the cb from inside a closure gets sticky)
    if let Some(ref s) = cargo_path {
        cb = cb.add_resource_path(s);
}
    let mut ctx = &mut cb.build().unwrap();
	let myGame = &mut Game::new(None,ctx);

	if let Err(e) = event::run(ctx, myGame) {
		println!("Error encountered: {}", e);
	}
	else {
		println!("Game exited cleanly.");
	}
}

extern crate ggez;
extern crate rand;
extern crate warmy;
extern crate specs;
extern crate ggwp_zscene as scene;

use ggez::conf;
use ggez::event;
use ggez::event::*;
use ggez::{Context, ContextBuilder, GameResult};
use std::path;
//use ggez::graphics::{self, Point2, Rect};

#[macro_use]
extern crate specs_derive;

//Modules for ECS
mod components;
mod system;
mod world;

mod screen;

struct Game{
	screens: screen::Screens,
}

impl Game {
	fn new(resource_dir: Option<path::PathBuf>, context: &mut Context) -> GameResult<Game>{
		let mut world = world::World::new(context, resource_dir.clone());
		let ocean = Box::new(screen::Ocean::new(context, &mut world)?);
		let screens = screen::Screens::new(ocean);
		let mut this = Game { 
			screens 
		};
		
		Ok(this)
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		self.screens.update(context)
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		self.screens.draw(context)
	}
}

fn main() {
    let mut cb = ContextBuilder::new("ld42", "icefoxen")
        .window_setup(conf::WindowSetup::default().title("Running In To Space"))
        .window_mode(conf::WindowMode::default().dimensions(800, 600));

    // We add the CARGO_MANIFEST_DIR/resources to the filesystems paths so
    // we we look in the cargo project for files.
    // And save it so we can feed there result into warmy

    let cargo_path: Option<path::PathBuf> = option_env!("CARGO_MANIFEST_DIR").map(|env_path| {
        let mut res_path = path::PathBuf::from(env_path);
        res_path.push("resources");
        res_path
    });
    // If we have such a path then add it to the context builder too
    // (modifying the cb from inside a closure gets sticky)
    if let Some(ref s) = cargo_path {
        cb = cb.add_resource_path(s);
}
    let mut context = cb.build().unwrap();
    
    match Game::new(None, &mut context) {
    	Err(e) => {
    		print!("Error: {}",e)
    	}
    	Ok(ref mut game) => {
    		let result = run(&mut context, game);
    		if let Err(e) = result{

    		} else {
    			
    		}
    	}
    }
}

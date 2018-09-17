extern crate ggez;
extern crate ggwp_zgui as ui;
extern crate ggwp_zscene as scene;
extern crate rand;
extern crate specs;

use ggez::conf;
use ggez::event;
use ggez::event::*;
use ggez::{Context, ContextBuilder, GameResult};
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
	fn new(context: &mut Context) -> GameResult<Game>{
		let mut world = world::World::new(context);
		let ocean = Box::new(screen::Ocean::new(context, &mut world)?);
		let screens = screen::Screens::new(ocean);
		let mut this = Game { screens };
		
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


fn context() -> Context {
	let window_conf = conf::WindowSetup::default()
		.resizable(true)
		.title("Polar Bear Revenge"); // WIP TITLE, CHANGE LATER
	ContextBuilder::new("polar", "sourdile")
		.window_setup(window_conf)
		.window_mode(conf::WindowMode::default().dimensions(1280, 720))
		.add_resource_path("assets")
		.build()
		.expect("Can't build context")
}

fn main() {
    let mut context = context();
    
    match Game::new(&mut context) {
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

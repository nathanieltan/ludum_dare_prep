use std::time::Duration;

use ggez::graphics::{self, Font, Point2, Text};
use ggez::{GameResult,Context};
use scene::{Layer
, Scene};

use screen::{self, Screen, Transition};

use world::*;
use components::*;


#[derive(Debug, Clone, Default)]
struct Layers {
	fg: Layer,
}

impl Layers {
	fn sorted(self) -> Vec<Layer> {
		vec![self.fg]
	}
}

#[derive(Debug)]
pub struct Ocean{
	layers: Layers,
	scene: Scene,
}

impl Ocean{
	pub fn new(context: &mut Context, world: &mut World) -> GameResult<Self>{
		let layers = Layers::default();
		let scene = Scene::new(layers.clone().sorted());

		Ok(Self {
			layers,
			scene,
		})
	}
}

impl Screen for Ocean{
	fn update(&mut self, _context: &mut Context, dtime: Duration) -> GameResult<Transition> {
		self.scene.tick(dtime);
		Ok(Transition::None)
	}

	fn draw(&self, context: &mut Context) -> GameResult<()> {
		self.scene.draw(context)
	}
}
use std::time::Duration;

use ggez::graphics::{self, Font, Point2, Text};
use ggez::{GameResult,Context};
use scene::{Layer, Scene, Sprite};
use ui::{self, Gui};

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
	world: World,
}

impl Ocean{
	pub fn new(context: &mut Context, world: &mut World) -> GameResult<Self>{

		let mut test_sprite = Sprite::from_path(context, "/testSprite.png", 360.0)?;
		//sprite.set_centered(true);
		test_sprite.set_pos(Point2::new(0.0,0.0));

		world.specs_world.create_entity.with(MySprite(test_sprite));

		let layers = Layers::default();
		let scene = Scene::new(layers.clone().sorted());

		Ok(Self {
			world,
			scene,
			layers,
		})
	}
}

impl Screen for Ocean{
	fn update(&mut self, _context: &mut Context, dtime: Duration) -> GameResult<Transition> {
		self.scene.tick(dtime);
		Ok(Transition::None)
	}

	fn draw(&self, context: &mut Context) -> GameResult<()> {
		self.sprite.draw(context)?;
		self.scene.draw(context)
	}
}
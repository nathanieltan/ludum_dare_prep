use ggez;
use ggez::graphics;
use ggez_goodies::scene;

use rand;
use specs::{self, Builder, Join};
use warmy;

use std::f32;

use components::*;
use input;
use scenes::*;
use systems::*;
use world::World;

pub struct LevelScene {
    done: bool,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;
        let dispatcher = Self::register_systems();

        LevelScene {
            done,
            dispatcher,
        }
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new()
            .build()
    }
}

impl scene::Scene<World, input::InputEvent> for LevelScene {
    fn update(&mut self, gameworld: &mut World) -> FSceneSwitch {
        if self.done {
            scene::SceneSwitch::Pop
        }
        else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        Ok(())
    }

    fn name(&self) -> &str {
        "LevelScene"
    }

    fn input(&mut self, gameworld: &mut World, _ev: input::InputEvent, _started: bool) {
        if gameworld.input.get_button_pressed(input::Button::Menu) {
            gameworld.quit = true;
        }
    }      
}
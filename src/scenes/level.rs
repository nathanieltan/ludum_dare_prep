use ggez;
use ggez::graphics;
use ggez_goodies::scene;
use nalgebra as na;
use ncollide2d as nc;
use rand;
use specs::{self, Builder, Join};
use warmy;

use std::f32;

use components::*;
use error::Err;
use input;
use resources;
use scenes::*;
use systems::*;
use util::*;
use world::World;

pub struct LevelScene {
    done: bool,
    player_entity: specs::Entity,
    dispatcher: specs::Dispatcher<'static, 'static>,
    background_mesh: graphics::Mesh,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Result<Self, Err> {
        let done = false;
        let dispatcher = Self::register_systems();
        let background_mesh = Self::create_background_mesh(ctx)?;
        let player_entity = Self::create_player(ctx, world)?;
        Ok(LevelScene {
            done,
            player_entity,
            dispatcher,
            background_mesh,
        })
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        specs::DispatcherBuilder::new()
            .build()
    }

    fn create_background_mesh(ctx: &mut ggez::Context) -> Result<graphics::Mesh, Err> {
        let num_stars = 10000;
        let star_max_bounds = 10000.0;
        let mut mb = graphics::MeshBuilder::new();
        for _ in 0..num_stars {
            let x = rand::random::<f32>() * star_max_bounds - (star_max_bounds / 2.0);
            let y = rand::random::<f32>() * star_max_bounds - (star_max_bounds / 2.0);
            mb.circle(
                graphics::DrawMode::Fill,
                graphics::Point2::new(x, y),
                2.0,
                2.0,
            );
        }
        mb.build(ctx).map_err(Err::from)
    }

    fn create_player(
        ctx: &mut ggez::Context,
        world: &mut World,
    ) -> Result<specs::Entity, Err> {
        let player_halfwidth = 8.0;
        let player_halfheight = 16.0;
        let player_offset = player_halfheight*3.0;
        let run_acceleration = 0.005;

        // Make the player entity
        let entity = world
            .specs_world
            .create_entity()
            .with(Player {
                on_ground: false,
                jumping: false,
                jump_force: 3.0,
                velocity: 0.0,
                run_acceleration,
                tumbling_timer: 0.0,
                friction: 0.0,
            })
            .with(Motion {
                velocity: Vector2::new(1.5, 0.0),
                acceleration: Vector2::new(0.0, 0.0),
            })
            .with(Mass {})
            // .with(Sprite {})
.build();
        
        Ok(entity)
    }
}

impl scene::Scene<World, input::InputEvent> for LevelScene {
    fn update(&mut self, gameworld: &mut World) -> FSceneSwitch {
        self.dispatcher.dispatch(&mut gameworld.specs_world.res);
        if self.done {
            scene::SceneSwitch::Pop
        }
        else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        graphics::draw(ctx, &self.background_mesh, ggez::nalgebra::origin(), 0.0)?;
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
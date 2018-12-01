use ggez::graphics;
use specs::*;
use util::*;


/// ///////////////////////////////////////////////////////////////////////
/// Components
/// ///////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Position(pub Point2);

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Motion {
    pub velocity: Vector2,
    pub acceleration: Vector2,
}

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Size(pub Point2);

/// Objects without one won't et affected by the 'Gravity' system
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Mass{}

/// Just a marker that a particular entity is the player.
#[derive(Clone, Debug, Component)]
#[storage(HashMapStorage)]
pub struct Player {
    pub on_ground: bool,
    pub jumping: bool,
    pub jump_force: f32,
    pub velocity: f32,
    pub run_acceleration: f32,
    pub tumbling_timer: f32,
    pub friction: f32,
}

/// Sprite marker.
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Sprite {
    //image: warmy::Res<resources::Image>,
}

/// Mesh
#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Mesh {
    pub mesh: graphics::Mesh,
}
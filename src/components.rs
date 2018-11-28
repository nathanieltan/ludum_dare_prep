use ggez::graphics::*;
use specs::*;


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

#[derive(Clone, Debug, Component)]
#[storage(VecStorage)]
pub struct Mass(pub Point2);
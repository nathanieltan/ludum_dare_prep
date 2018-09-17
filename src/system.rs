use specs::{self, Join};

use components::*;

pub struct MovementSystem;

impl<'a> specs::System<'a> for MovementSystem {
	type SystemData = (
		specs::WriteStorage<'a, Position>,
		specs::ReadStorage<'a, Motion>,
	);

	fn run(&mut self, (mut pos, motion): Self::SystemData) {
		for (pos, motion) in (&mut pos, &motion).join() {
			pos.0 += motion.velocity;
		}
	}
}
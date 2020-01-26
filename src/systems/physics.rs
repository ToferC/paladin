use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, ReadStorage, System, SystemData, World, WriteStorage};

use crate::paladin::{Physical, ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Physical>,
    );

    fn run(&mut self, (mut transforms, physicals): Self::SystemData) {
        for (transform, physical) in (&mut transforms, &physicals).join() {

            transform.prepend_translation_x(physical.velocity[0]);
            transform.prepend_translation_y(physical.velocity[1]);


            // wrap arena
            let physical_x = transform.translation().x;
            let physical_y = transform.translation().y;

            // top edge
            if (physical_y <= physical.radius && physical.velocity[1] < 0.0)
            || (physical_y >= ARENA_HEIGHT - physical.radius && physical.velocity[1] > 0.0)
            {
                transform.translation_mut().y = 0.0 - physical.radius;
            }

            // bottom edge
            if (physical_y >= physical.radius + ARENA_HEIGHT && physical.velocity[1] > 0.0)
            || (physical_y <= physical.radius && physical.velocity[1] < 0.0)
            {
                transform.translation_mut().y = ARENA_HEIGHT + physical.radius;
            }

        }
    }
}
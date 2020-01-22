use amethyst::core::{Transform, SystemDesc, math};
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::paladin::{Physical, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Physical>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, physicals, time): Self::SystemData) {
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

            // left edge
            if (physical_x <= physical.radius && physical.velocity[0] < 0.0)
            || (physical_x >= ARENA_WIDTH - physical.radius && physical.velocity[0] > 0.0)
            {
                transform.translation_mut().x = 0.0 - physical.radius;
            }

            // right edge
            if (physical_x >= physical.radius + ARENA_WIDTH && physical.velocity[0] > 0.0)
            || (physical_x <= physical.radius && physical.velocity[0] < 0.0)
            {
                transform.translation_mut().x = ARENA_WIDTH + physical.radius;
            }
        }
    }
}
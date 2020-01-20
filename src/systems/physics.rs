use amethyst::core::{Transform, SystemDesc, math};
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::paladin::{Ship, Side, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, time): Self::SystemData) {
        for transform in &mut transforms {

            

            transform.prepend_translation_x(ship.velocity[0]);
            transform.prepend_translation_y(ship.velocity[1]);


            // wrap arena
            let ship_x = transform.translation().x;
            let ship_y = transform.translation().y;

            // top edge
            if (ship_y <= ship.height && ship.velocity[1] < 0.0)
            || (ship_y >= ARENA_HEIGHT - ship.height && ship.velocity[1] > 0.0)
            {
                transform.translation_mut().y = 0.0 - ship.height;
            }

            // bottom edge
            if (ship_y >= ship.height + ARENA_HEIGHT && ship.velocity[1] > 0.0)
            || (ship_y <= ship.height && ship.velocity[1] < 0.0)
            {
                transform.translation_mut().y = ARENA_HEIGHT + ship.height;
            }

            // left edge
            if (ship_x <= ship.width && ship.velocity[0] < 0.0)
            || (ship_x >= ARENA_WIDTH - ship.width && ship.velocity[0] > 0.0)
            {
                transform.translation_mut().x = 0.0 - ship.height;
            }

            // right edge
            if (ship_x >= ship.width + ARENA_WIDTH && ship.velocity[0] > 0.0)
            || (ship_x <= ship.width && ship.velocity[0] < 0.0)
            {
                transform.translation_mut().x = ARENA_WIDTH + ship.width;
            }


        }
    }
}
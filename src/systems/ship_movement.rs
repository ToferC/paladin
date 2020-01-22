use amethyst::core::{Transform, SystemDesc, math};
use amethyst::core::timing::Time;
use amethyst::core::math::Vector3;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, Entities, Entity, LazyUpdate, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::paladin::{Ship, Side, Laser, ARENA_HEIGHT, ARENA_WIDTH, LASER_RADIUS};

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ship>,
        WriteStorage<'s, Laser>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, mut transforms, mut ships, mut lasers, input, time, lazy): Self::SystemData) {

        for (ship, transform) in (&mut ships, &mut transforms).join() {

            let movement = match ship.side {
                Side::Light => input.axis_value("rotate"),
                _ => None,
            };
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = ship.agility * mv_amount as f32;
                    transform.rotate_2d(scaled_amount);
                }
            }

            let thrust = match ship.side {
                Side::Light => input.axis_value("accelerate"),
                _ => None,
            };

            if let Some(thrust) = thrust {
                let added = math::Vector3::y() * ship.acceleration * time.delta_seconds() * thrust;
                let added = transform.rotation() * added;
                ship.velocity[0] += added.x;
                ship.velocity[1] += added.y;

            }


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

        for (ship, transform) in (&ships, &mut transforms).join() {
            transform.prepend_translation_x(ship.velocity[0]);
            transform.prepend_translation_y(ship.velocity[1]);
        }
    }
}
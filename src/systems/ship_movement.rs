use amethyst::core::{Transform, SystemDesc, math};
use amethyst::core::timing::Time;
use amethyst::core::math::Vector3;
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, Entities, Entity, LazyUpdate, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::paladin::{Ship, Side, Laser, Physical, ARENA_HEIGHT, ARENA_WIDTH, LASER_RADIUS};

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physical>,
        WriteStorage<'s, Ship>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (mut transforms, mut physicals, mut ships, input, time, lazy): Self::SystemData) {

        for (ship, transform, physical) in (&mut ships, &mut transforms, &mut physicals).join() {

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
                physical.velocity[0] += added.x;
                physical.velocity[1] += added.y;

            }
        }
    }
}
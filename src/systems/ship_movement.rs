use amethyst::core::{Transform, SystemDesc, math};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::paladin::{Ship, Side, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Ship>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, ships, input): Self::SystemData) {
        for (ship, transform) in (&ships, &mut transforms).join() {
            let movement = match ship.side {
                Side::Light => input.axis_value("rotate"),
                _ => None,
            };
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = ship.agility * mv_amount as f32;
                    println!("Moving: {}", scaled_amount);
                    transform.rotate_2d(scaled_amount);
                }
            }

            let thrust = match ship.side {
                Side::Light => input.axis_value("accelerate"),
                _ => None,
            };

            if let Some(acceleration) = thrust {
                let added = math::Vector3::y() * ship.speed * acceleration as f32;
                transform.append_translation(added);
                println!("Thrust activated!");
            }
        }
    }
}
use amethyst::core::{Transform, SystemDesc, math};
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
};
use amethyst::ecs::{Join, Read, ReadExpect, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::paladin::{Ship, Side, Physical};
use crate::audio::{play_thrust_sound, Sounds};

use std::ops::Deref;

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physical>,
        WriteStorage<'s, Ship>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,

        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (
        mut transforms, 
        mut physicals, 
        mut ships, 
        input, 
        time, 

        // audio
        storage,
        sounds,
        audio_output,
     ): Self::SystemData) {
         
        for (ship, transform, physical) in (&mut ships, &mut transforms, &mut physicals).join() {

            let movement = match ship.side {
                Side::Light => input.axis_value("rotate"),
                _ => None,
            };
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    let scaled_amount = physical.agility * mv_amount as f32;
                    transform.rotate_2d(scaled_amount);
                }
            }

            let thrust = match ship.side {
                Side::Light => input.axis_value("accelerate"),
                _ => None,
            };

            let drift = match ship.side {
                Side::Light => input.axis_value("drift"),
                _ => None,
            };


            if let Some(thrust) = thrust {

                if thrust != 0.0 {
                    let added = math::Vector3::y() * physical.acceleration * time.delta_seconds() * thrust;
                    let added = transform.rotation() * added;
                    physical.velocity[0] += added.x;
                    physical.velocity[1] += added.y;
    
                    if ship.thrust_timer <= 0.0 {
                        // play SFX
                        play_thrust_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
                        ship.thrust_timer = 2.0;
                    } else {
                        ship.thrust_timer -= time.delta_seconds();
                    }
                }
            }

            if let Some(drift) = drift {
                if drift > 0.0 {
                    transform.move_left(physical.acceleration * drift);
                } else if drift < 0.0 {
                    transform.move_left(physical.acceleration * drift);
                }
            }
        }
    }
}
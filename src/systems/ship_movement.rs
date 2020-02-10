use amethyst::core::{Transform, SystemDesc, math};
use amethyst::core::timing::Time;
use amethyst::derive::SystemDesc;
use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
};
use amethyst::ecs::{Join, Entities, LazyUpdate, Read, ReadExpect, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::audio::{play_thrust_sound, Sounds};
use crate::components::{Ship, Side, Physical};
use crate::components::{Thrust, ThrustRes, show_thrust};
use crate::resources::{AssetType, SpriteSheetList};

use std::ops::Deref;

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physical>,
        WriteStorage<'s, Ship>,
        ReadExpect<'s, SpriteSheetList>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,

        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        ReadExpect<'s, LazyUpdate>,
    );

    fn run(&mut self, (
        entities,
        mut transforms, 
        mut physicals, 
        mut ships,
        sprite_sheet_list,
        input, 
        time, 

        // audio
        storage,
        sounds,
        audio_output,
        lazy,
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

                if thrust > 0.0 {

                    let added = math::Vector3::y() * physical.acceleration * time.delta_seconds() * thrust;
                    let added = transform.rotation() * added;
                    physical.velocity += math::Vector2::new(added.x, added.y);

                    // limit velocity
                    let magnitude = physical.velocity.magnitude();

                    if magnitude > physical.max_velocity {
                        physical.velocity /= magnitude / physical.max_velocity;
                    }

                    // Timer for basic sound effects
                    if ship.thrust_timer <= 0.0 {
                        // play SFX
                        play_thrust_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
                        ship.thrust_timer = 2.0;
                    } else {
                        ship.thrust_timer -= time.delta_seconds();
                    }

                    // show thruster
                    let thrust_spritesheet_handle = {
                        sprite_sheet_list.get(AssetType::Thrust).unwrap().clone()
                    };

                    let ship_transform = transform.clone();

                    /*

                    show_thrust(
                        &entities,
                        thrust_spritesheet_handle,
                        &lazy,
                        ship_transform,
                    );
                    */
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
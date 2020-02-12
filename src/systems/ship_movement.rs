use amethyst::core::{Transform, SystemDesc, math, Hidden, Parent};
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
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        WriteStorage<'s, Thrust>,
        WriteStorage<'s, Hidden>,
        WriteStorage<'s, Parent>,

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
        input, 
        time,
        mut thrust_entities,
        mut hidden_entities,
        mut parents,

        // audio
        storage,
        sounds,
        audio_output,
        lazy,
     ): Self::SystemData) {
         
        for (entity, ship, transform, physical) in (&entities, &mut ships, &mut transforms, &mut physicals).join() {

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

                // Get thrust entity so we can apply and remove the Hidden tag
                let thrust_entity = (&entities, &parents).join()
                    .find_map(|(ent, parent)| if parent.entity == entity {
                    Some(ent)
                } else {
                    None
                });

                let thrust_entity = thrust_entity.unwrap();

                if thrust > 0.0 {
                    let added = math::Vector3::y() * physical.acceleration * time.delta_seconds() * thrust;
                    let added = transform.rotation() * added;
                    physical.velocity += math::Vector2::new(added.x, added.y);

                    // limit velocity
                    let magnitude = physical.velocity.magnitude();

                    if magnitude > physical.max_velocity {
                        physical.velocity /= magnitude / physical.max_velocity;
                    }

                    // Remove Hidden tag
                    hidden_entities.remove(thrust_entity);

                    // Timer for basic sound effects
                    if ship.thrust_timer <= 0.0 {
                        // play SFX
                        play_thrust_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
                        ship.thrust_timer = 2.0;
                    } else {
                        ship.thrust_timer -= time.delta_seconds();
                    }


                } else {
                    // No thrust - add hidden tag
                    hidden_entities.insert(thrust_entity, Hidden).expect("");
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
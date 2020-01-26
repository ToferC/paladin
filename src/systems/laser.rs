use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::timing::Time,
    core::transform::Transform,
    core::math::{Vector3, Vector2},
    core::SystemDesc,
    derive::SystemDesc,
    input::{InputHandler, StringBindings},
    ecs::prelude::{Join, Read, ReadExpect, Entities, ReadStorage, System, SystemData, World, WriteStorage, LazyUpdate},
};

use std::ops::Deref;
use smallvec::SmallVec;

use crate::paladin::{Ship, Side, Combat, Laser, LaserRes, Physical};
use super::audio::{play_laser_sound, Sounds};

#[derive(SystemDesc)]
pub struct LaserSystem;

impl<'s> System<'s> for LaserSystem {

    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, LaserRes>,
        WriteStorage<'s, Laser>,
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Ship>,
        WriteStorage<'s, Combat>,
        WriteStorage<'s, Transform>,

        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,

        Read<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run(&mut self, (
        entities, 
        laser_resource, 
        mut lasers, 
        input, 
        ships, 
        mut combats, 
        mut transforms,
        storage,
        sounds,
        audio_output,
        lazy, 
        time): Self::SystemData) {

        for (ship, transform, combat) in (&ships, &mut transforms, &mut combats).join() {
            // does ship shoot?
            let shoot = match ship.side {
                Side::Light => input.action_is_down("shoot").unwrap_or(false),
                _ => false,
            };

            let mut new_lasers = SmallVec::<[NewLaser; 8]>::new();

            if combat.reload_timer <= 0.0 {
                if shoot {
                    println!{"PEW PEW"};

                    combat.reload_timer = combat.time_to_reload;
    
                    let velocity = transform.rotation() * Vector3::y() * combat.laser_velocity;
    
                    let mut laser_t = transform.clone();
                    
                    laser_t.append_translation(Vector3::new(0.0, 80.0, 0.0));
    
                    laser_t.set_scale(Vector3::new(4.0, 4.0, 0.0));

                    let mut physical = Physical::new(8.0, 4.0);
                    physical.velocity = Vector2::new(velocity.x, velocity.y);
                    
                    // set timer here for burst fire
                    new_lasers.push(NewLaser {
                        laser_t: laser_t.clone(),
                        physical: physical.clone(),
                    });
                    
                };
            } else {
                combat.reload_timer -= time.delta_seconds();

                if combat.reload_timer <= 0.0 {
                    combat.reload_timer = 0.0;
                }
            }

            if !new_lasers.is_empty() {
                play_laser_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
            }

            for new_laser in new_lasers {

                let NewLaser { laser_t, physical } = new_laser;

                let laser = Laser::new(combat.laser_timer, combat.laser_damage, ship.side);
    
                let e = entities.create();

                lazy.insert(e, laser);
                lazy.insert(e, physical);
                lazy.insert(e, laser_t);
                lazy.insert(e, laser_resource.sprite_render());

            }
        }


        // laser kill
        for (entity, laser) in (&entities, &mut lasers).join() {

            laser.timer -= time.delta_seconds();

            if laser.timer <= 0.0 {
                // time up, remove laser
                entities.delete(entity).expect("Couldn't delete entity");
                println!("Laser gone!")
            } else {
                // update timer
                laser.timer -= time.delta_seconds();
            }
        }

        struct NewLaser {
            laser_t: Transform,
            physical: Physical,
        }
    }
}
use amethyst::{
    assets::{AssetStorage, Handle, Prefab},
    audio::{output::Output, Source},
    core::timing::Time,
    core::transform::Transform,
    core::math::{Vector3, Vector2, UnitQuaternion, Translation3},
    core::SystemDesc,
    derive::SystemDesc,
    renderer::{transparent::Transparent, SpriteRender},
    input::{InputHandler, StringBindings},
    ecs::prelude::{Entity, Join, Read, ReadExpect, Entities, ReadStorage, System, SystemData, World, WriteStorage, LazyUpdate},
};

use std::ops::Deref;
use smallvec::SmallVec;

use crate::resources::{SpriteSheetList, AssetType};
use crate::audio::{play_laser_sound, Sounds};
use crate::components::{LaserImpact, Laser, Ship, Side};
use crate::components::{Animation, AnimationId, AnimationPrefabData};
use crate::components::{Physical, Combat, LaserType};

#[derive(SystemDesc)]
pub struct LaserSystem;

impl<'s> System<'s> for LaserSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Laser>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, SpriteSheetList>,
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
        mut lasers, 
        input,
        sprite_sheet_list,
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
                Side::Light => input.action_is_down("light_shoot").unwrap_or(false),
                Side::Dark => input.action_is_down("dark_shoot").unwrap_or(false),
            };

            let mut new_lasers = SmallVec::<[NewLaser; 8]>::new();

            if combat.reload_timer <= 0.0 {
                if shoot {

                    combat.reload_timer = combat.time_to_reload;
    
                    let velocity = transform.rotation() * Vector3::y() * combat.laser_velocity;
    
                    let mut laser_t = transform.clone();

                    
                    // Shoot single high-power laser
                    laser_t.append_translation(Vector3::new(0.0, 80.0, 0.0));
    
                    laser_t.set_scale(Vector3::new(4.0, 4.0, 0.0));

                    let mut physical = Physical::new(8., 4., 0., 0.);
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

                match combat.laser_type {
                    LaserType::Single => {
                        let NewLaser { laser_t, physical } = new_laser;
        
                        let laser = Laser::new(combat.laser_timer, combat.laser_damage, ship.side);

                        let light_laser_sprite_sheet_handle = sprite_sheet_list.get(AssetType::LaserLight).unwrap();

                         // Construct sprite render for light_laser
                        let light_laser_sprite_render = SpriteRender {
                            sprite_sheet: light_laser_sprite_sheet_handle.clone(),
                            sprite_number: 0,
                        };
            
                        let e = entities.create();
        
                        lazy.insert(e, laser);
                        lazy.insert(e, physical);
                        lazy.insert(e, laser_t);
                        lazy.insert(e, light_laser_sprite_render.clone());
                    }
                    LaserType::Dual => {
                        // Laser 1
                        let NewLaser { mut laser_t, physical } = new_laser;

                        laser_t.append_translation(Vector3::new(-25.0, 0.0, 0.0));
                        laser_t.set_scale(Vector3::new(2., 2., 2.));

                        let laser = Laser::new(combat.laser_timer, combat.laser_damage, ship.side);

                        let dark_laser_sprite_sheet_handle = sprite_sheet_list.get(AssetType::LaserDark).unwrap();

                        // Construct sprite render for thruster
                        let dark_laser_sprite_render = SpriteRender {
                            sprite_sheet: dark_laser_sprite_sheet_handle.clone(),
                            sprite_number: 0,
                        };
            
                        let e = entities.create();
        
                        lazy.insert(e, laser.clone());
                        lazy.insert(e, physical);
                        lazy.insert(e, laser_t.clone());
                        lazy.insert(e, dark_laser_sprite_render.clone());

                        // Laser 2

                        laser_t.append_translation(Vector3::new(50.0, 0.0, 0.0));

                        let f = entities.create();
        
                        lazy.insert(f, laser.clone());
                        lazy.insert(f, physical);
                        lazy.insert(f, laser_t.clone());
                        lazy.insert(f, dark_laser_sprite_render.clone());
                    }
                }

            }
        }


        // laser kill
        for (entity, laser) in (&entities, &mut lasers).join() {

            laser.timer -= time.delta_seconds();

            if laser.timer <= 0.0 {
                // time up, remove laser
                entities.delete(entity).expect("Couldn't delete entity");
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

pub fn show_laser_impact(
    entities: &Entities,
    prefab_handle: Handle<Prefab<AnimationPrefabData>>,
    mut transform: Transform,
    lazy_update: &ReadExpect<LazyUpdate>,
) {
    let laser_impact_entity: Entity = entities.create();

    transform.rotate_2d(-0.8);
    transform.set_scale(Vector3::new(0.7, 0.7, 0.7));

    lazy_update.insert(laser_impact_entity, LaserImpact::default());
    lazy_update.insert(
        laser_impact_entity,
        Animation::new(AnimationId::LaserImpact, vec![AnimationId::LaserImpact]),
    );
    lazy_update.insert(laser_impact_entity, prefab_handle);
    lazy_update.insert(laser_impact_entity, transform);
    lazy_update.insert(laser_impact_entity, Transparent);
}
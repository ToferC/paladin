use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, Read, ReadExpect, Entities, System, SystemData, World, WriteStorage, LazyUpdate},
};

use std::ops::Deref;

use crate::paladin::{RandomGen, LASER_RADIUS};
use crate::audio::{play_impact_sound, Sounds};
use crate::systems::laser::show_laser_impact;
use crate::components::{Laser, Ship, Physical, Combat};
use crate::resources::{PrefabList, AssetType};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Laser>,
        ReadStorage<'s, Ship>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physical>,
        WriteStorage<'s, Combat>,
        ReadExpect<'s, RandomGen>,

        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
        ReadExpect<'s, LazyUpdate>,
        ReadExpect<'s, PrefabList>,
    );

    fn run(&mut self, (
        entities, 
        lasers, 
        ships, 
        mut transforms, 
        mut physicals, 
        mut combat, 
        random_gen, 

        storage,
        sounds,
        audio_output,
        lazy_update,
        prefab_list,
    ): Self::SystemData) {
        
        // laser collision
        for (laser, entity) in (&lasers, &entities).join() {

            // get laser coordinates
            let (laser_x, laser_y) = {
                let trans = transforms.get(entity).expect("Unable to load laser transform");
                let l_x = trans.translation().x;
                let l_y = trans.translation().y;
                (l_x, l_y)
            };

            // get laser velocity - used for impact to ship velocity
            let laser_vel = {
                let phys = physicals.get(entity).expect("Unable to load laser physical");
                phys.velocity.clone()
            };

            let laser_trans = {
                let trans = transforms.get(entity).expect("unable to load laser transform");
                trans.clone()
            };

            for (ship, ship_transform, combat, physical) in (&ships, &mut transforms, &mut combat, &mut physicals).join() {
                let ship_x = ship_transform.translation().x;
                let ship_y = ship_transform.translation().y;

                if circles_collide(
                    laser_x,
                    laser_y,
                    LASER_RADIUS,

                    ship_x,
                    ship_y,
                    physical.radius,
                ) {
                    // damage ship hit
                    let mut damage = laser.damage - combat.armour;
                    if damage <= 0 {
                        damage = 0;
                    };

                    combat.structure -= damage;
                    play_impact_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));

                    let laser_impact_prefab_handle = {
                        prefab_list.get(AssetType::LaserImpact).unwrap().clone()
                    };

                    show_laser_impact(
                        &entities,
                        laser_impact_prefab_handle,
                        laser_x,
                        laser_y,
                        ship_x,
                        ship_y,
                        &lazy_update,
                    );

                    if combat.structure <= 0 {
                        // explode ship and delete
                        println!("{:?} ship is vaporized!", ship.side);
                    } else {
                        // adjust & jitter ship vector based on impact

                        ship_transform.rotate_2d(random_gen.next_f32() - 0.5);

                        // push ship
                        physical.velocity[0] += laser_vel[0] * 0.01;
                        physical.velocity[1] += laser_vel[1] * 0.01;
                    }
                    // delete laser
                    entities.delete(entity).expect("Unable to delete laser");
                }
            }
        }
        // check for ship collisions
        let mut ships_iter = (&ships, &mut physicals, &mut combat, &mut transforms).join();
        
        let (light_ship, mut light_physical, mut light_combat, light_transform) = ships_iter.next().unwrap();
        let (dark_ship, mut dark_physical, mut dark_combat, dark_transform) = ships_iter.next().unwrap();

        if circles_collide(
            light_transform.translation().x,
            light_transform.translation().y,
            light_physical.radius,

            dark_transform.translation().x,
            dark_transform.translation().y,
            dark_physical.radius,
        ) {

            // track impact on light & dark ships
            let mut light_damage = 30 - light_combat.armour;
            if light_damage <= 0 {
                light_damage = 0;
            };

            light_combat.structure -= light_damage;

            let mut dark_damage = 30 - dark_combat.armour;
            if dark_damage <= 0 {
                dark_damage = 0;
            };

            dark_combat.structure -= dark_damage;

            play_impact_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));
             
            if light_combat.structure <= 0 {
                // explode ship and delete
                println!("{:?} ship is vaporized!", light_ship.side);
            } else {
                // adjust & jitter ship vector based on impact

                light_transform.rotate_2d(random_gen.next_f32() - 0.5);


                light_physical.velocity[0] = -light_physical.velocity[0];
                light_physical.velocity[1] = -light_physical.velocity[1];

               

            }

            if dark_combat.structure <= 0 {
                // explode ship and delete
                println!("{:?} ship is vaporized!", dark_ship.side);
            } else {
                // adjust & jitter ship vector based on impact

                dark_transform.rotate_2d(random_gen.next_f32() - 0.5);

                dark_physical.velocity[0] = -dark_physical.velocity[0];
                dark_physical.velocity[1] = -dark_physical.velocity[1];

            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y>= bottom && y <= top
}

fn circles_collide(a_x: f32, a_y: f32, a_r: f32, b_x: f32, b_y: f32, b_r: f32) -> bool {

    let dx = a_x - b_x;
    let dy = a_y - b_y;

    let distance = dx * dx + dy * dy;

    if distance > (a_r * a_r) + (b_r * b_r) {
        false
    } else {
        true
    }
}
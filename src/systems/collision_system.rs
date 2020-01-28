use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ui::UiText,
    ecs::prelude::{Join, ReadStorage, Read, ReadExpect, Entities, System, SystemData, World, WriteStorage},
};

use std::ops::Deref;

use crate::paladin::{Laser, Physical, Ship, Side, Combat, RandomGen, StructureText ,LASER_RADIUS};
use crate::audio::{play_impact_sound, Sounds};

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
        WriteStorage<'s, UiText>,
        ReadExpect<'s, StructureText>,

        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(&mut self, (
        entities, 
        lasers, 
        ships, 
        mut transforms, 
        mut physicals, 
        mut combat, 
        random_gen, 
        mut ui_text, 
        struct_text,

        storage,
        sounds,
        audio_output,
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

            for (ship_entity, ship, ship_transform, combat, physical) in (&entities, &ships, &mut transforms, &mut combat, &mut physicals).join() {
                let ship_x = ship_transform.translation().x;
                let ship_y = ship_transform.translation().y;

                if point_in_rect(
                    laser_x,
                    laser_y,
                    ship_x - LASER_RADIUS, // left
                    ship_y - LASER_RADIUS, // bottom
                    ship_x + ship.width + LASER_RADIUS, // right
                    ship_y + ship.height + LASER_RADIUS, // top
                ) {
                    println!("Hit!");

                    // damage ship hit
                    let mut damage = laser.damage - combat.armour;
                    if damage <= 0 {
                        damage = 0;
                    };

                    combat.structure -= damage;
                    println!("Hit for {} damage - {} left!", damage, combat.structure);
                    play_impact_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));

                    // Update HP tracker
                    match ship.side {
                        Side::Light => {
                            if let Some(text) = ui_text.get_mut(struct_text.light_struct_text) {
                                text.text = format!("HP: {}", combat.structure);
                            }
                        }
                        Side::Dark => {
                            if let Some(text) = ui_text.get_mut(struct_text.dark_struct_text) {
                                text.text = format!("HP: {}", combat.structure);
                            }
                        }
                    }

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
                    println!("Laser expended");
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y>= bottom && y <= top
}
use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, Entities, System, SystemData, World, WriteStorage},
};

use crate::paladin::{Laser, Physical, Ship, LASER_RADIUS};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Laser>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Physical>,
        Entities<'s>,
    );

    fn run(&mut self, (lasers, ships, mut transforms, mut physicals, entities): Self::SystemData) {
        
        // laser collision
        for (laser, transform, entity) in (&lasers, &transforms, &entities).join() {
            let laser_x = transform.translation().x;
            let laser_y = transform.translation().y;

            let phys = physicals.get(entity).expect("Unable to load laser physical");
            let l_vel = phys.velocity.clone();
            drop(phys);

            for (ship, ship_transform, physical) in (&ships, &transforms, &mut physicals).join() {
                let ship_x = ship_transform.translation().x;
                let ship_y = ship_transform.translation().y;

                if point_in_rect(
                    laser_x,
                    laser_y,
                    ship_x - LASER_RADIUS,
                    ship_y - LASER_RADIUS,
                    ship_x + ship.width + LASER_RADIUS,
                    ship_y + ship.height + LASER_RADIUS,
                ) {
                    println!("Hit!");

                    // adjust ship vector based on impact
                    physical.velocity[0] += phys.velocity[0] * 0.001;
                    physical.velocity[1] += phys.velocity[1] * 0.001;

                    // delete laser
                    entities.delete(entity).expect("Unable to delete laser");
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y>= bottom && y <= top
}
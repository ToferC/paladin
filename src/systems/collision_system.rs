use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::prelude::{Join, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::paladin::{Laser, Ship, LASER_RADIUS};

#[derive(SystemDesc)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        ReadStorage<'s, Laser>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (lasers, ships, transforms): Self::SystemData) {
        for (laser, transform) in (&lasers, &transforms).join() {
            let laser_x = transform.translation().x;
            let laser_y = transform.translation().y;

            for (ship, ship_transform) in (&ships, &transforms).join() {
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
                    println!("Hit!")
                }
            }
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y>= bottom && y <= top
}
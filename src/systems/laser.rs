use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    input::{InputHandler, StringBindings},
    ecs::prelude::{Join, Read, Entities, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::paladin::{Ship, Side, Laser, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct LaserSystem;

impl<'s> System<'s> for LaserSystem {

    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Laser>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, mut lasers, mut transforms, time): Self::SystemData) {

        // move every laser according to speed and time passed
        for (entity, laser, transform) in (&entities, &mut lasers, &mut transforms).join() {

            laser.timer -= time.delta_seconds();

            if laser.timer <= 0.0 {
                // time up, remove laser
                entities.delete(entity).expect("Couldn't delete entity");
                println!("Laser gone!")
            } else {
                // update timer
                laser.timer -= time.delta_seconds();

                // update velocity
                transform.prepend_translation_x(laser.velocity[0] * time.delta_seconds());
                transform.prepend_translation_y(laser.velocity[1] * time.delta_seconds());            
                
                // wrap arena
                let laser_x = transform.translation().x;
                let laser_y = transform.translation().y;
        
                // top edge
                if (laser_y <= laser.radius && laser.velocity[1] < 0.0)
                || (laser_y >= ARENA_HEIGHT - laser.radius && laser.velocity[1] > 0.0)
                {
                    transform.translation_mut().y = 0.0 - laser.radius;
                }
        
                // bottom edge
                if (laser_y >= laser.radius + ARENA_HEIGHT && laser.velocity[1] > 0.0)
                || (laser_y <= laser.radius && laser.velocity[1] < 0.0)
                {
                    transform.translation_mut().y = ARENA_HEIGHT + laser.radius;
                }
        
                // left edge
                if (laser_x <= laser.radius && laser.velocity[0] < 0.0)
                || (laser_x >= ARENA_WIDTH - laser.radius && laser.velocity[0] > 0.0)
                {
                    transform.translation_mut().x = 0.0 - laser.radius;
                }
        
                // right edge
                if (laser_x >= laser.radius + ARENA_WIDTH && laser.velocity[0] > 0.0)
                || (laser_x <= laser.radius && laser.velocity[0] < 0.0)
                {
                    transform.translation_mut().x = ARENA_WIDTH + laser.radius;
                }
            }
        }
    }
}
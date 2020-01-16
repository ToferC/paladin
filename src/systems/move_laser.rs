use amethyst::{
    core:timing::Time,
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
};

use crate::paladin::Laser;

#[derive(SystemDesc)]
pub struct MoveLaserSystem;

impl<'s> System<'s> for MoveLaserSystem {
    type SystemData = (
        ReadStorage<'s, Laser>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (lasers, mut locals, time): Self::SystemData) {
        // move every laser according to speed and time passed
        for (laser, local) in (lasers, &mut locals).join() {
            locals.prepend_translation_x(laser.velocity[0] * time.delta_seconds());
            locals.prepend_translation_y(laser.velocity[1] * time.delta_seconds());            
        }
    }
}
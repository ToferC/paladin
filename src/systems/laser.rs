use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::math::{Vector3, Vector2},
    core::SystemDesc,
    derive::SystemDesc,
    input::{InputHandler, StringBindings},
    ecs::prelude::{Join, Read, ReadExpect, Entities, ReadStorage, System, SystemData, World, WriteStorage, LazyUpdate},
};

use crate::paladin::{Ship, Side, Combat, Laser, LaserRes, Physical};

#[derive(SystemDesc)]
pub struct LaserSystem;

impl<'s> System<'s> for LaserSystem {

    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, LaserRes>,
        WriteStorage<'s, Laser>,
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Ship>,
        ReadStorage<'s, Combat>,
        WriteStorage<'s, Transform>,
        Read<'s, LazyUpdate>,
        Read<'s, Time>,
    );

    fn run(&mut self, (entities, laser_resource, mut lasers, input, ships, combats, mut transforms, lazy, time): Self::SystemData) {

        for (ship, transform, combat) in (&ships, &mut transforms, &combats).join() {
            // does ship shoot?
            let shoot = match ship.side {
                Side::Light => input.action_is_down("shoot").unwrap_or(false),
                _ => false,
            };

            if shoot {
                println!{"PEW PEW"};

                let velocity = transform.rotation() * Vector3::y() * combat.laser_velocity;

                let mut laser_t = transform.clone();
                
                laser_t.append_translation(Vector3::new(0.0, 55.0, 0.0));

                laser_t.set_scale(Vector3::new(4.0, 4.0, 0.0));

                let mut physical = Physical::new(16.0, 1.0);
                physical.velocity = Vector2::new(velocity.x, velocity.y);

                let laser = Laser::new(combat.laser_timer, combat.laser_damage, ship.side);

                let e = entities.create();

                lazy.insert(e, laser);
                lazy.insert(e, physical);
                lazy.insert(e, laser_t);
                lazy.insert(e, laser_resource.sprite_render());
                };
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
    }
}
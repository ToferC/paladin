use amethyst::{
    core::timing::Time,
    core::transform::Transform,
    core::math::{Vector3, Vector2},
    core::SystemDesc,
    derive::SystemDesc,
    input::{InputHandler, StringBindings},
    ecs::prelude::{Join, Read, ReadExpect, Entities, ReadStorage, System, SystemData, World, WriteStorage, LazyUpdate},
};

use crate::paladin::{Ship, Side, Laser, LaserRes, Physical, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct LaserSystem;

impl<'s> System<'s> for LaserSystem {

    type SystemData = (
        Entities<'s>,
        ReadExpect<'s, LaserRes>,
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Ship>,
        WriteStorage<'s, Transform>,
        Read<'s, LazyUpdate>,
    );

    fn run(&mut self, (entities, laser_resource, input, ships, mut transforms, lazy): Self::SystemData) {

        for (ship, transform) in (&ships, &mut transforms).join() {
            // does ship shoot?
            let shoot = match ship.side {
                Side::Light => input.action_is_down("shoot").unwrap_or(false),
                _ => false,
            };

            if shoot {
                println!{"PEW PEW"};

                let velocity = transform.rotation() * Vector3::y() * ship.laser_velocity;
                
                let mut physical = Physical::new(16.0);
                physical.velocity = Vector2::new(velocity.x, velocity.y);

                let laser = Laser::new();

                let e = entities.create();

                lazy.insert(e, laser);
                lazy.insert(e, physical);
                lazy.insert(e, transform.clone());
                lazy.insert(e, laser_resource.sprite_render());
                };
        }
    }
}
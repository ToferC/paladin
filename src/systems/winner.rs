use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    core::math::Vector2,
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, World, WriteStorage},
};

use crate::paladin::{Ship, Physical, Side, ARENA_WIDTH, ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ship>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physical>
    );

    fn run(&mut self, (mut ships, mut locals, mut physicals): Self::SystemData) {

        let mut did_hit = false;

        for (ship, transform) in (&mut ships, &mut locals).join() {

            did_hit = match ship.side {
                Side::Light => {
                    let ship_x = transform.translation().x;

                    if ship_x <= ship.width {
                        // Right player scores
                        println!("Player 2 scores!");
                        true
                    } else {
                        false
                    }
                }
                Side::Dark => {
                    let ship_x = transform.translation().x;

                    if ship_x >= ARENA_WIDTH - ship.width {
                        // Left player scores
                        println!("Player 1 scores!");
                        true
                    } else {
                        false
                    }
                }
            };
        }

        if did_hit {
            // reset physics and reposition ships
            // Correctly position the ships.
            let y = ARENA_HEIGHT / 2.0;

            for (ship, transform, physical) in (&mut ships, &mut locals, &mut physicals).join() {
                match ship.side {
                    Side::Light => {
                        transform.set_translation_xyz(ship.width * 3.0, y, 0.0);
                        physical.velocity = Vector2::new(0.0, 0.0);
                        physical.rotation = 0.0;
                        // rotate ships
                        transform.rotate_2d(1.60);
                    }
                    Side::Dark => {
                        transform.set_translation_xyz(ARENA_WIDTH - ship.width * 3.0, y, 0.0);
                        physical.velocity = Vector2::new(0.0, 0.0);
                        physical.rotation = 0.0;
                        // rotate ships
                        transform.rotate_2d(-1.60);
                    }
                }
            }
        }
    }
}
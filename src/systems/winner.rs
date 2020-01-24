use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    derive::SystemDesc,
    ecs::prelude::{Join, System, SystemData, World, WriteStorage},
};

use crate::paladin::{Ship, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        WriteStorage<'s, Ship>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (mut ships, mut locals): Self::SystemData) {
        for (ship, transform) in (&mut ships, &mut locals).join() {
            let ship_x = transform.translation().x;

            let did_hit = if ship_x <= ship.width {
                // Right player scores
                println!("Player 2 scores!");
                true
            } else if ship_x >= ARENA_WIDTH - ship.width {
                // left player scores
                println!("Player 1 scores!");
                true
            } else {
                false
            };

            if did_hit {

            }

        }
    }
}
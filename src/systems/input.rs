use amethyst::{
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::Ship;

pub struct InputSystem;

impl<'s> System for InputSystem {
    type SystemData = (
        WriteStorage<'s, Ship>
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut ships, input) = data;

        for ship in &mut ships {
            
        }
    }

}
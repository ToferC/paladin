use amethyst::{
    core::{Transform, math},
    ecs::{Entities, Entity, LazyUpdate, ReadExpect, DenseVecStorage, Component},
    renderer::{sprite::SpriteSheetHandle, transparent::Transparent, SpriteRender},
    prelude::*,
};

use crate::components::{Physical, Ship};

use crate::resources::assets::load_sprite_sheet;

#[derive(Debug)]
pub struct Thrust {
    pub show: bool,
}

impl Component for Thrust {
    type Storage = DenseVecStorage<Self>;
}

pub struct ThrustRes {
    pub sprite_render: SpriteRender
}

impl Component for ThrustRes {
    type Storage = DenseVecStorage<Self>;
}

impl ThrustRes {
    pub fn initialise(world: &mut World) {
        let sprite_sheet_handle = load_sprite_sheet(world, "texture/thrust");

        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet_handle,
            sprite_number: 0,
        };

        world.insert(ThrustRes { sprite_render });
    }

    pub fn sprite_render(&self) -> SpriteRender {
        self.sprite_render.clone()
    }
}

pub fn show_thrust(
    entities: &Entities,
    sprite_sheet_handle: SpriteSheetHandle,
    lazy: &ReadExpect<LazyUpdate>,
    mut transform: Transform,
    mut physical: Physical,
) {
    let e : Entity = entities.create();

    transform.append_translation(math::Vector3::new(0.0, -20.0, 0.0));
    //transform.set_scale(math::Vector3::new(0.2, 0.2, 0.2));

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0,
    };

    transform.rotate_2d(-1.6);

    lazy.insert(e, transform);
    lazy.insert(e, physical);
    lazy.insert(e, Transparent);
    //lazy.insert(e, Thrust::default());
    lazy.insert(e, sprite_render);
}
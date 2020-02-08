extern crate specs_derive;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader, RonFormat, ProgressCounter},
    derive::PrefabData,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
};

use crate::paladin::{AnimationId, AnimationPrefabData};



pub struct ExplosionRes {
    pub animation_id: AnimationId,
    pub animation_prefab_handle: Handle<Prefab<AnimationPrefabData>>,
}

impl Component for ExplosionRes {
    type Storage = DenseVecStorage<Self>;
}

impl ExplosionRes {
    pub fn initialise(world: &mut World, progress_counter: &mut ProgressCounter) {
        let animation_prefab_handle = world.exec(|loader: PrefabLoader<'_, AnimationPrefabData>| {
            loader.load("prefab/laser_impact.ron", RonFormat, progress_counter)
        });

        let animation_id = AnimationId::Explosion;

        world.insert(ExplosionRes { animation_id, animation_prefab_handle });
    }

    pub fn animation_render(&self) -> Handle<Prefab<AnimationPrefabData>> {
        self.animation_prefab_handle.clone()
    }
}
use amethyst::{
    animation::AnimationSetPrefab,
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    error::Error,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    renderer::{SpriteRender,
        sprite::{prefab::SpriteScenePrefab},
    },
};

use serde::{Serialize, Deserialize};

/// Animation
#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    LaserImpact,
}

#[derive(Clone, Debug, Deserialize, PrefabData)]
pub struct AnimationPrefabData {
    sprite_scene: SpriteScenePrefab,
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}

/// https://github.com/amethyst/space-menace/blob/master/src/components/animation.rs
#[derive(Debug)]
pub struct Animation {
   pub current: AnimationId,
   pub types: Vec<AnimationId>,
   pub show: bool,
}

impl Animation {
    pub fn new(current: AnimationId, types: Vec<AnimationId>) -> Self {
        Self {
            current,
            types,
            show: true,
        }
    }
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}
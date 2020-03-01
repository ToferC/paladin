use amethyst::{
    core::transform::ParentHierarchy,
    ecs::{
        error::WrongGeneration,
        prelude::{Entity, World, WorldExt},
    },
};

use std::iter;

// delete specified root entity and all descendants as specified by Parent component
pub fn delete_hierarchy(root: Entity, world: &mut World) -> Result<(), WrongGeneration> {
    let entities = {
        iter::once(root)
            .chain(
                world
                    .read_resource::<ParentHierarchy>()
                    .all_children_iter(root),   
            )
            .collect::<Vec<Entity>>()
    };
    world.delete_entities(&entities)
}
use amethyst::{
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    derive::SystemDesc,
    renderer::{
        plugins::{RenderFlat2D},
        sprite::SpriteRender,
    },
    ecs::{WriteStorage, Read, ReadStorage, Entities, System, Join},
};

use crate::paladin::{Animation, Explosion, AnimationId};

pub struct ExplosionAnimationSystem;

impl<'s> System<'s> for ExplosionAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Explosion>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, (
        entities, 
        explosions, 
        mut animations, 
        mut animation_control_sets,
    ): Self::SystemData) {
        
        for (entity, _, mut animation, animation_control_set) in (
            &entities,
            &explosions, 
            &mut animations, 
            &mut animation_control_sets).join() {
            
                if animation.show {
                animation_control_set.start(animation.current);
                animation.show = false;
            } else {
                let explode_animation = animation_control_set
                    .animations
                    .iter()
                    .find(|(id, _)| *id == AnimationId::Explosion);

                if explode_animation.is_none() {
                    let _ = entities.delete(entity);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct AnimationControlSystem;

impl<'s> System<'s> for AnimationControlSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Animation>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, animations, animation_sets, mut animation_control_sets) = data;

        for (entity, animation, animation_set) in (&entities, &animations, &animation_sets).join() {
            
            let animation_control_set = get_animation_set(&mut animation_control_sets, entity).unwrap();

            if animation.show {
                animation.types.iter().for_each(|animation_id| {
                    if !animation_control_set.has_animation(*animation_id) {
                        println!(
                            "Added animation with id {:?} for entity {:?}",
                            animation_id,
                            entity,
                        );

                        let end = match animation_id {
                            AnimationId::Explosion => EndControl::Stay
                        };
                        animation_control_set.add_animation(
                            *animation_id,
                            &animation_set.get(&animation_id).unwrap(),
                            end,
                            1.0,
                            AnimationCommand::Init,  
                        );
                    }
                });
            }

            // start the animation for the current animation ID
            animation_control_set.start(animation.current);
        }
    }
}
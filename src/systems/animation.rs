use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    renderer::{
        sprite::SpriteRender,
    },
    ecs::{WriteStorage, ReadStorage, Entities, System, Join},
};

use crate::components::{LaserImpact, Animation, AnimationId};

pub struct LaserImpactAnimationSystem;

impl<'s> System<'s> for LaserImpactAnimationSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, LaserImpact>,
        WriteStorage<'s, Animation>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(&mut self, (
        entities, 
        laser_impacts, 
        mut animations, 
        mut animation_control_sets,
    ): Self::SystemData) {
        
        for (entity, _, mut animation, animation_control_set) in (
            &entities,
            &laser_impacts, 
            &mut animations,
            &mut animation_control_sets,
        )
            .join() 
        {
            if animation.show {
                animation_control_set.start(animation.current);
                animation.show = false;
            } else {
                let laser_impact_animation = animation_control_set
                    .animations
                    .iter()
                    .find(|(id, _)| *id == AnimationId::LaserImpact);

                if laser_impact_animation.is_none() {
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
            
            // fetch or create the AnimationControlSet for this entity
            let animation_control_set = get_animation_set(&mut animation_control_sets, entity).unwrap();

            if animation.show {
                animation.types.iter().for_each(|&animation_id| {
                    if !animation_control_set.has_animation(animation_id) {
                        
                        let end = match animation_id {
                            AnimationId::LaserImpact => EndControl::Stay,
                            _ => EndControl::Loop(None),
                        };
                        animation_control_set.add_animation(
                            animation_id,
                            &animation_set.get(&animation_id).unwrap(),
                            end,
                            1.,
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
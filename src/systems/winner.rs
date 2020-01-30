use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    core::math::Vector2,
    derive::SystemDesc,
    ui::UiText,
    ecs::prelude::{Join, System, SystemData, World, Write, WriteStorage, ReadStorage, ReadExpect, Entities},
};

use crate::paladin::{Ship, Physical, Combat, Side, ScoreBoard, ScoreText, ARENA_WIDTH, ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Ship>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physical>,
        WriteStorage<'s, Combat>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (entities, ships, mut locals, mut physicals, mut combats, mut ui_text, mut scores, score_text): Self::SystemData) {

        let mut is_destroyed: bool;

        for (entity, ship) in (&entities, &ships).join() {

            let structure = combats.get(entity).unwrap().structure;

            is_destroyed = match ship.side {
                Side::Light => {

                    if structure <= 0 {
                        // Right player scores
                        scores.score_dark = (scores.score_dark + 1)
                            .min(999);

                        if let Some(text) = ui_text.get_mut(score_text.dark_text) {
                            text.text = scores.score_dark.to_string();
                        }
                        // Return true
                        true
                    } else {
                        false
                    }
                }
                Side::Dark => {
                    if structure <= 0 {
                        // Left player scores
                        scores.score_light = (scores.score_light + 1)
                            .min(999);

                        if let Some(text) = ui_text.get_mut(score_text.light_text) {
                            text.text = scores.score_light.to_string();
                        }
                        // return true
                        true
                    } else {
                        false
                    }
                }
            };

            if is_destroyed {
                // reset physics and reposition ships
                // Correctly position the ships.
                let y = ARENA_HEIGHT / 2.0;
    
                for (ship, transform, physical, combat) in (&ships, &mut locals, &mut physicals, &mut combats).join() {
                    match ship.side {
                        Side::Light => {
                            transform.set_translation_xyz(physical.radius * 4.0, y, 0.0);
                            physical.velocity = Vector2::new(0.0, 0.0);
                            physical.rotation = 0.0;
                            // rotate ships
                            transform.rotate_2d(1.60);
                            combat.structure = 150;
                        }
                        Side::Dark => {
                            transform.set_translation_xyz(ARENA_WIDTH - physical.radius * 4.0, y, 0.0);
                            physical.velocity = Vector2::new(0.0, 0.0);
                            physical.rotation = 0.0;
                            // rotate ships
                            transform.rotate_2d(-1.60);
                            combat.structure = 150;
                        }
                    }
                }
                println!(
                    "Score: | {:^3} | {:^3} |",
                    scores.score_light, scores.score_dark
                );
            }
        }
    }
}
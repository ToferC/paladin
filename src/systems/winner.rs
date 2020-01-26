use amethyst::{
    core::transform::Transform,
    core::SystemDesc,
    core::math::Vector2,
    derive::SystemDesc,
    ui::UiText,
    ecs::prelude::{Join, System, SystemData, World, Write, WriteStorage, ReadStorage, ReadExpect, Entities},
};

use crate::paladin::{Ship, Physical, Side, ScoreBoard, ScoreText, ARENA_WIDTH, ARENA_HEIGHT};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Ship>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physical>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
    );

    fn run(&mut self, (entities, ships, mut locals, mut physicals, mut ui_text, mut scores, score_text): Self::SystemData) {

        let mut did_hit: bool;

        for (entity, ship) in (&entities, &ships).join() {

            did_hit = match ship.side {
                Side::Light => {

                    let light_trans = locals.get(entity).clone().unwrap();
                    let ship_x = light_trans.translation().x;

                    if ship_x <= ship.width {
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
                    let dark_trans = locals.get(entity).clone().unwrap();
                    let ship_x = dark_trans.translation().x;

                    if ship_x >= ARENA_WIDTH - ship.width {
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

            if did_hit {
                // reset physics and reposition ships
                // Correctly position the ships.
                let y = ARENA_HEIGHT / 2.0;
    
                for (ship, transform, physical) in (&ships, &mut locals, &mut physicals).join() {
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
                println!(
                    "Score: | {:^3} | {:^3} |",
                    scores.score_light, scores.score_dark
                );
            }
        }
    }
}
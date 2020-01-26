use amethyst::{
    assets::Loader,
    audio::{OggFormat, SourceHandle},
    ecs::{World, WorldExt},
};

use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source},
};

const LASER_SOUND: &str = "audio/laser.ogg";
const THRUST_SOUND: &str = "audio/thrust.ogg";
const IMPACT_SOUND: &str = "audio/impact.ogg";
const EXPLOSION_SOUND: &str = "audio/explosion.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";

pub struct Sounds {
    pub laser_sfx: SourceHandle,
    pub thrust_sfx: SourceHandle,
    pub impact_sfx: SourceHandle,
    pub explosion_sfx: SourceHandle,
    pub score_sfx: SourceHandle,
}

/// Loads an ogg audio track
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

/// Initialize audio in the world.
pub fn initialize_audio(world: &mut World) {
    let sound_effects = {
        let loader = world.read_resource::<Loader>();

        let sound = Sounds {
            laser_sfx: load_audio_track(&loader, &world, LASER_SOUND),
            thrust_sfx: load_audio_track(&loader, &world, THRUST_SOUND),
            impact_sfx: load_audio_track(&loader, &world, IMPACT_SOUND),
            explosion_sfx: load_audio_track(&loader, &world, EXPLOSION_SOUND),
            score_sfx: load_audio_track(&loader, &world, SCORE_SOUND),
        };

        sound
    };
    // add sound effects to world in another scope
    world.insert(sound_effects);
}

pub fn play_laser_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.laser_sfx) {
            output.play_once(sound, 1.0)
        }
    }
}

pub fn play_thrust_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.thrust_sfx) {
            output.play_once(sound, 1.0)
        }
    }
}

pub fn play_impact_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.impact_sfx) {
            output.play_once(sound, 1.0)
        }
    }
}
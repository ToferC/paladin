

pub struct SimpleAnimation {
    start_sprite_index: usize,
    frames: usize,
    current_frame: usize,
    time_per_frame: f32,
    elapsed_time: f32,
    active: bool,
}

impl SimpleAnimation {
    pub fn new(start_sprite_index: usize, frames: usize, time_per_frame: f32,) -> SimpleAnimation {
        SimpleAnimation {
            start_sprite_index,
            frames,
            current_frame: 0,
            time_per_frame,
            elapsed_time: 0.0,
            active: true,
        }
    }
}
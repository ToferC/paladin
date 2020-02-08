use amethyst::ecs::{DenseVecStorage, Component};

/// Combat represents damage, defense and attack in the game
#[derive(Debug)]
pub struct Combat {
    pub structure: i32,
    pub armour: i32,
    // lasers
    pub laser_damage: i32,
    pub laser_timer: f32,
    pub laser_velocity: f32,
    pub reload_timer: f32,
    pub time_to_reload: f32,
    pub burst_rate: i32,

    pub burst_delay: f32,
    pub burst_timer: f32,

    pub missile_damage: i32,
    pub missile_timer: f32,
    pub missile_explosion_radius: f32,
    pub missile_velocity: f32,
}

impl Component for Combat {
    type Storage = DenseVecStorage<Self>;
}

impl Combat {
    pub fn new(
        structure: i32,
        armour: i32, 
        laser_damage: i32,
        laser_timer: f32,
        laser_velocity: f32,


        missile_damage: i32,
        missile_timer: f32,
        missile_explosion_radius: f32,
        missile_velocity: f32,
    ) -> Combat {
            Combat {
                structure,
                armour,

                laser_damage,
                laser_timer,
                laser_velocity,
                reload_timer: 0.0,
                time_to_reload: 0.2,
                burst_rate: 8,

                burst_delay: 0.05,
                burst_timer: 0.0,

                missile_damage,
                missile_timer,
                missile_explosion_radius,
                missile_velocity,
            }
        }
}
